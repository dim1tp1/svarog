// tailored version
// 1 master key splits into n additive key shares
// master key derived from an imported phrase, len of {12, 15, 18, 21, 24}
// chain code of master key strictly in accord with BIP32
use std::ops::Deref;

use crate::{
    aes_decrypt, aes_encrypt, algo::check_dir_rwx, Batch, MpcRound, RuntimeKeyStore,
    ToVecByKeyOrder, AEAD,
};
use bip32::{ChildNumber, ExtendedKey, ExtendedKeyAttrs, Prefix}; // chain_code = left half of SHA512(pk)
use bip39::{Language, Mnemonic, Seed};
use bitcoin::{network::constants::Network, util::bip32::ExtendedPrivKey};
use curv::{
    arithmetic::traits::Converter,
    cryptographic_primitives::{
        commitments::{hash_commitment::HashCommitment, traits::Commitment},
        proofs::sigma_dlog::DLogProof,
        secret_sharing::feldman_vss::VerifiableSS,
    },
    elliptic::curves::{secp256_k1::Secp256k1, Point, Scalar},
    BigInt,
};
use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2018::party_i::{
    self as kzen, KeyGenBroadcastMessage1, KeyGenDecommitMessage1, Keys,
};
use paillier::EncryptionKey;
use sha2::Sha256;
use xuanmi_base_support::*;

use crate::lbm_client::{LubanManagerClient, TerminationType};
use lx_grpc::prelude::tokio::sync::mpsc;
use lx_grpc::{
    lbcs::{mpc_progress::Harvest, KeyMetadata, MpcProgress},
    prelude::tonic::Status,
};
type KeygenT = (String, RuntimeKeyStore); // (shard_mnemonic, shard_keystore)
type OutputStream<T> = mpsc::Sender<Result<T, Status>>;

pub fn scalar_split(num: &Scalar<Secp256k1>, count: &usize) -> Vec<Scalar<Secp256k1>> {
    let mut partition: Vec<Scalar<Secp256k1>> = Vec::new();
    for _j in 0..count - 1 {
        partition.push(Scalar::<Secp256k1>::random());
    }
    let partial_sum: Scalar<Secp256k1> = partition.iter().sum();
    partition.push(num - partial_sum);
    partition
}

#[tracing::instrument(skip(ostream, keystore_dir))]
pub async fn algo_keygen_root_mnem(
    ostream: &OutputStream<MpcProgress>,
    keystore_dir: &str,
    owner_id: &str,
    server: &str,
    session_id: &str,
    phrase: &str,
    password: &str,
) -> Outcome<()> {
    // check permission of keystore_dir
    let keystore_dir = keystore_dir.to_lexical_abspath().catch_()?;
    check_dir_rwx(&keystore_dir).await.catch_()?;

    let mut messenger = LubanManagerClient::attend(server, session_id, owner_id)
        .await
        .catch_()?;
    let mut progress = MpcProgress {
        current: 0,
        total: 12,
        message: String::new(),
        harvest: None,
    };
    let my_id: usize = messenger.party_id();
    let n_actual: usize = messenger.n_actual();
    let n_keygen: usize = messenger.n_keygen();
    let threshold: usize = messenger.threshold();
    assert_throw!(n_keygen > threshold && n_keygen == n_actual);
    let ex = &format!("KeygenException ({}-{})", session_id, my_id);
    let config = kzen::Parameters {
        threshold: threshold as u16,
        share_count: n_keygen as u16,
    };
    let if_master: bool = !phrase.is_empty();
    if !(if_master || password.is_empty()) {
        throw!(ex, "Password assigned to non-master!");
    }
    progress.message("KRM-Attend.").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::info!(
        "party_id={}, th={}, n_keygen={}",
        my_id,
        threshold,
        n_keygen
    );

    // preround 0: collect master ID
    let mut round = 1;
    messenger
        .send_broadcast(round, &(if_master as u16))
        .await
        .catch_()?;
    let mut batch_if_master: Batch<u16> = messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_if_master.insert(my_id, if_master as u16);

    // check if there is only one 1 in the values of batch_if_master hashmap,
    // if not, throw exception
    // if there is only one 1, then the corresponding key is the master id
    if batch_if_master.values().sum::<u16>() != 1 {
        throw!(ex, "Should be exactly 1 phrase!");
    }
    let master_id: usize = *batch_if_master.iter().find(|&(_, v)| *v == 1).unwrap().0;
    round += 1;
    progress.message("KRM-Master").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::info!("Root key provider is party_id={}", master_id);
    // #end-preround0

    // preround 1: send commitment to ephemeral public keys
    let mut party_keys = Keys::create(my_id as u16);
    let mut expected_y_sum = Point::<Secp256k1>::generator().to_point();
    let (mut com_i, mut decom_i) = party_keys.phase1_broadcast_phase3_proof_of_correct_key();
    messenger.send_broadcast(round, &com_i).await.catch_()?;
    let mut batch_com: Batch<KeyGenBroadcastMessage1> =
        messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_com.insert(my_id, com_i);
    round += 1;
    progress.message("KRM1").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Received broadcast of commitments from all parties.");
    // #end-preround1

    // preround 2: exchange ephemeral public keys
    messenger.send_broadcast(round, &decom_i).await.catch_()?;
    let mut batch_decom1: Batch<KeyGenDecommitMessage1> =
        messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_decom1.insert(my_id, decom_i);
    let batch_aeskey: Batch<BigInt> = batch_decom1
        .iter()
        .map(|(id, decom)| {
            let id = *id;
            let val = (decom.y_i.clone() * party_keys.u_i.clone())
                .x_coord()
                .unwrap();
            (id, val)
        })
        .collect();
    let correct_key_correct_decom_all = (1..=n_keygen).all(|i| {
        HashCommitment::<Sha256>::create_commitment_with_user_defined_randomness(
            &BigInt::from_bytes(batch_decom1.get(&i).unwrap().y_i.to_bytes(true).as_ref()),
            &batch_decom1.get(&i).unwrap().blind_factor,
        ) == batch_com.get(&i).unwrap().com
            && batch_com
                .get(&i)
                .unwrap()
                .correct_key_proof
                .verify(
                    &batch_com.get(&i).unwrap().e,
                    zk_paillier::zkproofs::SALT_STRING,
                )
                .is_ok()
    });
    if !correct_key_correct_decom_all {
        throw!(ex, "Invalid key or commitment!");
    }
    round += 1;
    progress.message("KRM2").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Check key and decommitment correctness");
    // #end-preround2

    let chain_code = if if_master {
        let seed = Seed::new(
            &Mnemonic::from_phrase(phrase, Language::English).catch_()?,
            password,
        );
        let seed_bytes: &[u8] = seed.as_bytes();
        let master_sk = ExtendedPrivKey::new_master(Network::Bitcoin, seed_bytes).catch_()?;
        expected_y_sum = &Scalar::<Secp256k1>::from_bytes(&master_sk.private_key.secret_bytes())
            .unwrap()
            * Point::<Secp256k1>::generator();
        let chain_code = master_sk.chain_code.to_bytes();
        let partition = scalar_split(
            &Scalar::<Secp256k1>::from_bytes(&master_sk.private_key.secret_bytes()).unwrap(),
            &n_keygen,
        );
        // preround 3: send secret shares via aes-p2p
        for (k, i) in (1..=n_keygen).enumerate() {
            if i != my_id {
                // prepare encrypted share for party i
                let key_i = BigInt::to_bytes(batch_aeskey.get(&i).unwrap());
                let mut plaintext = BigInt::to_bytes(&partition[k].to_bigint());
                let aead_pack_i1 = aes_encrypt(&key_i, &plaintext).catch_()?;
                plaintext = BigInt::to_bytes(&BigInt::from_bytes(&chain_code));
                let aead_pack_i2 = aes_encrypt(&key_i, &plaintext).catch_()?;
                messenger
                    .send_p2p(i, round, &(aead_pack_i1, aead_pack_i2))
                    .await
                    .catch_()?;
            }
        }
        (party_keys.u_i, party_keys.y_i) = (
            partition[my_id - 1].clone(),
            &partition[my_id - 1] * Point::<Secp256k1>::generator(),
        );
        chain_code
    } else {
        let (aead_pack_i1, aead_pack_i2): (AEAD, AEAD) =
            messenger.recv_p2p(master_id, round).await.catch_()?;

        let key_i = BigInt::to_bytes(batch_aeskey.get(&master_id).unwrap());
        let mut out = aes_decrypt(&key_i, &aead_pack_i1).catch_()?;
        let out_bn = BigInt::from_bytes(&out);
        let out_fe = Scalar::<Secp256k1>::from(&out_bn);
        (party_keys.u_i, party_keys.y_i) =
            (out_fe.clone(), &out_fe * Point::<Secp256k1>::generator());
        out = aes_decrypt(&key_i, &aead_pack_i2).catch_()?;

        // transform out from Vec<u8> to bip32::ChainCode type
        let mut chain_code_bytes = [0u8; 32];
        chain_code_bytes.copy_from_slice(&out);
        chain_code_bytes
    };
    round += 1;
    progress.message("KRM3").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchange aes-encrypted secret shares.");
    // #end-preround3

    // round 1: send commitment to ephemeral public keys
    (com_i, decom_i) = party_keys.phase1_broadcast_phase3_proof_of_correct_key();
    messenger.send_broadcast(round, &com_i).await.catch_()?;
    let mut batch_com: Batch<KeyGenBroadcastMessage1> =
        messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_com.insert(my_id, com_i);

    round += 1;
    progress.message("KRM4").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchange commitment to ephemeral public keys.");
    // #endregion

    // round 2: send ephemeral public keys
    messenger.send_broadcast(round, &decom_i).await.catch_()?;
    let mut batch_decom1: Batch<KeyGenDecommitMessage1> =
        messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_decom1.insert(my_id, decom_i);

    let batch_point: Batch<Point<Secp256k1>> = batch_decom1
        .iter()
        .map(|(k, v)| (*k, v.y_i.clone()))
        .collect();
    let batch_aeskey: Batch<BigInt> = batch_decom1
        .iter()
        .map(|(id, decom)| {
            (
                *id,
                (decom.y_i.clone() * party_keys.u_i.clone())
                    .x_coord()
                    .unwrap(),
            )
        })
        .collect();
    let root_pubkey = batch_point
        .iter()
        .fold(Point::<Secp256k1>::zero(), |sum, x| sum + x.1);
    round += 1;
    progress.message("KRM5.1").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchange ephemeral public keys.");
    // #endregion

    // check commitments correctness
    let (vss_scheme, secret_shares, _index) = {
        match party_keys.phase1_verify_com_phase3_verify_correct_key_phase2_distribute(
            &config,
            &batch_decom1.values_sorted_by_key_asc(),
            &batch_com.values_sorted_by_key_asc(),
        ) {
            Ok(_ok) => _ok,
            Err(_) => throw!(ex, "invalid key"),
        }
    };
    progress.message("KRM5.2").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Check commitments correctness.");

    // #region round 3: exchange secret shares in a peer-to-peer manner
    let mut batch_secret_shares: Batch<Scalar<Secp256k1>> = Batch::with_capacity(n_keygen);
    for party_id in 1..=n_keygen {
        let share = secret_shares[party_id - 1].clone();
        let _ = batch_secret_shares.insert(party_id, share);
    }
    for party_id in 1..=n_keygen {
        if party_id == my_id {
            continue;
        }
        let key = BigInt::to_bytes(batch_aeskey.get(&party_id).unwrap());
        let data = BigInt::to_bytes(&batch_secret_shares.get(&party_id).unwrap().to_bigint());
        let aead = aes_encrypt(&key, &data).catch_()?;
        messenger.send_p2p(party_id, round, &aead).await.catch_()?;
    }
    let batch_aead: Batch<AEAD> = messenger.recv_all_p2p(round).await.catch_()?;
    let mut batch_party_shares: Batch<Scalar<Secp256k1>> = Batch::with_capacity(n_keygen);
    let _ = batch_party_shares.insert(my_id, secret_shares[my_id - 1].clone());
    for party_id in 1..=n_keygen {
        if party_id == my_id {
            continue;
        }
        let key = batch_aeskey.get(&party_id).unwrap().to_bytes();
        let aead = batch_aead.get(&party_id).unwrap();
        let data = aes_decrypt(&key, aead).catch_()?;
        let data = BigInt::from_bytes(&data);
        let share: Scalar<Secp256k1> = Scalar::from(&data);
        let _ = batch_party_shares.insert(party_id, share);
    }
    round += 1;
    progress.message("KRM6").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchange secret shares in a peer-to-peer manner.");
    // #endregion

    // #region round 4: exchange vss commitments
    messenger
        .send_broadcast(round, &vss_scheme)
        .await
        .catch_()?;
    let mut batch_vss_scheme: Batch<VerifiableSS<Secp256k1>> =
        messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_vss_scheme.insert(my_id, vss_scheme.clone());
    round += 1;
    progress.message("KRM7.1").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchange vss commitments.");
    // #endregion

    // #region round5: exchange and verify exchange dlog proof
    let (shared_keys, dlog_proof) = {
        match party_keys.phase2_verify_vss_construct_keypair_phase3_pok_dlog(
            &config,
            &batch_point.values_sorted_by_key_asc(),
            &batch_party_shares.values_sorted_by_key_asc(),
            &batch_vss_scheme.values_sorted_by_key_asc(),
            my_id as u16,
        ) {
            Ok(_ok) => _ok,
            Err(__) => throw!(ex, "Invalid vss"),
        }
    };
    messenger
        .send_broadcast(round, &dlog_proof)
        .await
        .catch_()?;
    let mut batch_dlog_proof: Batch<DLogProof<Secp256k1, Sha256>> =
        messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_dlog_proof.insert(my_id, dlog_proof.clone());
    match kzen::Keys::verify_dlog_proofs(
        &config,
        &batch_dlog_proof.values_sorted_by_key_asc(),
        &batch_point.values_sorted_by_key_asc(),
    ) {
        Ok(_) => {}
        Err(_) => throw!(ex, "Bad dlog proof!"),
    }
    progress.message("KRM7.2").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Verified dlog proof.");
    // #endregion

    if if_master {
        assert!(root_pubkey == expected_y_sum, "public key not match!");
    }
    let mnemonic = Mnemonic::from_entropy(&party_keys.u_i.to_bytes(), Language::English)
        .catch(ex, "Cannot create mnemonic.")?;
    let shard_mnemonic: String = mnemonic.phrase().to_string();

    let batch_paillier_key: Batch<EncryptionKey> = batch_com
        .iter()
        .map(|(id, bc)| (*id, bc.e.clone()))
        .collect();

    // #region round6: terminate
    let root_xpub = {
        let pkb_short = root_pubkey.to_bytes(true).deref().to_vec();
        let ex_pk = ExtendedKey {
            prefix: Prefix::XPUB,
            attrs: ExtendedKeyAttrs {
                parent_fingerprint: [0u8; 4],
                child_number: ChildNumber(0u32),
                chain_code,
                depth: 0u8,
            },
            key_bytes: pkb_short.try_into().unwrap(),
        };
        ex_pk.to_string()
    };
    let termination = TerminationType::Keygen(root_xpub.clone());
    messenger.send_terminate(&termination).await.catch_()?;
    // #endregion

    let shard_keystore = RuntimeKeyStore {
        chain_code,
        root_pubkey,
        party_keys,
        shared_keys,
        batch_vss_scheme,
        batch_paillier_key,

        owner_id: owner_id.to_owned(),
        key_id: session_id.to_owned(),
    };
    let kspath = format!("{}/{}-{}.keystore", keystore_dir, owner_id, session_id)
        .to_lexical_abspath()
        .catch_()?;
    shard_keystore.marshall_proto_file(&kspath).await.catch_()?;

    progress
        .message("KRM-Fin")
        .harvest(Harvest::KeyMeta(KeyMetadata {
            root_xpub: root_xpub.clone(),
            shard_mnemonic,
        }))
        .increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::info!(
        "Keygen finished. \nRootXPub={}\nkspath={}. ",
        &root_xpub,
        &kspath
    );
    Ok(())
}
