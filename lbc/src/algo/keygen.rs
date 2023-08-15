use std::ops::Deref;

use bip32::{ChainCode, ChildNumber, ExtendedKey, ExtendedKeyAttrs, Prefix}; // chain_code = left half of SHA512(pk)
use bip39::{Language, Mnemonic};
use curv::{
    arithmetic::traits::Converter,
    cryptographic_primitives::{
        proofs::sigma_dlog::DLogProof, secret_sharing::feldman_vss::VerifiableSS,
    },
    elliptic::curves::{secp256_k1::Secp256k1, Point, Scalar},
    BigInt,
};
use lx_grpc::prelude::tokio::sync::mpsc;
use lx_grpc::{
    lbcs::{mpc_progress::Harvest, KeyMetadata, MpcProgress},
    prelude::tonic::Status,
};
use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2018::party_i::{
    self as kzen, KeyGenBroadcastMessage1, KeyGenDecommitMessage1,
};
use paillier::EncryptionKey;
use sha2::{Digest, Sha256, Sha512};
use xuanmi_base_support::*;

use crate::{
    aes_decrypt, aes_encrypt,
    algo::check_dir_rwx,
    lbm_client::{LubanManagerClient, TerminationType},
    Batch, MpcRound, RuntimeKeyStore, ToVecByKeyOrder, AEAD,
};
type OutputStream<T> = mpsc::Sender<Result<T, Status>>;

/// Generate an MPC keyshard.
///
/// # Arguments
/// * `owner_id` - String-ID of the owner of this keyshard. The owner can be a person or a hosted server.
/// * `server` - mpc server URL.
/// * `uuid_tr` - transaction UUID.
///   If keygen succeeds, this UUID will be used as the id of "true secret key".
/// * `threshold` - at least `threshold+1` keyshards to sign a message.
///
/// # Returns
/// * Ok((shard_mnemonic, shard_keystore)) or Err(Ex)
#[tracing::instrument(skip(ostream, keystore_dir))]
pub async fn algo_keygen(
    ostream: &OutputStream<MpcProgress>,
    keystore_dir: &str,
    owner_id: &str,
    server: &str,
    session_id: &str,
) -> Outcome<()> {
    // check permission of keystore_dir
    let keystore_dir = keystore_dir.to_lexical_abspath().catch_()?;
    check_dir_rwx(&keystore_dir).await.catch_()?;
    tracing::debug!("keystore_dir: {}", keystore_dir);

    let mut messenger = LubanManagerClient::attend(server, session_id, owner_id)
        .await
        .catch_()?;
    let mut progress = MpcProgress {
        current: 0,
        total: 7,
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

    progress.message("K-Attend").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!(
        "party_id={}, th={}, n_keygen={}",
        my_id,
        threshold,
        n_keygen
    );

    // #region round 1: exchange commitment to ephemeral public keys
    let mut round = 1;
    let party_keys = kzen::Keys::create(my_id as u16);
    let shard_mnemonic = Mnemonic::from_entropy(&party_keys.u_i.to_bytes(), Language::English)
        .catch(ex, "Cannot create mnemonic.")?;
    let shard_mnemonic: String = shard_mnemonic.phrase().to_string();
    let (bc_i, decom_i) = party_keys.phase1_broadcast_phase3_proof_of_correct_key();
    messenger.send_broadcast(round, &bc_i).await.catch_()?;
    let mut batch_bc1: Batch<KeyGenBroadcastMessage1> =
        messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_bc1.insert(my_id, bc_i);
    round += 1;
    progress.message("K1").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchanged commitment to ephemeral public keys.");
    // #endregion

    // #region round 2: exchange ephemeral public keys
    messenger.send_broadcast(round, &decom_i).await.catch_()?;
    let mut batch_decom1: Batch<KeyGenDecommitMessage1> =
        messenger.recv_broadcasts(round).await.catch_()?;
    let _ = batch_decom1.insert(my_id, decom_i.clone());
    let batch_point: Batch<Point<Secp256k1>> = batch_decom1
        .iter()
        .map(|(id, decom)| (*id, decom.y_i.clone()))
        .collect();
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
    let root_pubkey = batch_point
        .iter()
        .fold(Point::<Secp256k1>::zero(), |sum, x| sum + x.1);
    round += 1;
    progress.message("K2").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchanged commitment to ephemeral public keys.");
    // #endregion

    // #region round 3: exchange secret shares in a peer-to-peer manner
    let (vss_scheme, secret_shares, _index) = {
        match party_keys.phase1_verify_com_phase3_verify_correct_key_phase2_distribute(
            &config,
            &batch_decom1.values_sorted_by_key_asc(),
            &batch_bc1.values_sorted_by_key_asc(),
        ) {
            Ok(_ok) => _ok,
            Err(_) => throw!(ex, "invalid key"),
        }
    };
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
    progress.message("K3").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchanged secret shares.");
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
    progress.message("K4").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchanged vss commitments.");
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
    progress.message("K5").increase();
    ostream.send(Ok(progress.clone())).await.catch_()?;
    tracing::debug!("Exchange and verify dlog proof.");
    // #endregion

    // #region compose keystore json
    let batch_paillier_key: Batch<EncryptionKey> = batch_bc1
        .iter()
        .map(|(id, bc)| (*id, bc.e.clone()))
        .collect();
    let chain_code = {
        let pkb_long = root_pubkey.to_bytes(false).deref().to_vec();
        let chain_code: ChainCode = Sha512::digest(&pkb_long)
            .get(..32)
            .if_none(ex, "")?
            .try_into()
            .unwrap();
        chain_code
    };
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
    messenger
        .send_terminate(&TerminationType::Keygen(root_xpub.clone()))
        .await
        .catch_()?;
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
        .message("K-Fin")
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
