// batch_sign of Luban 1 by Jerry
// tailored version for batch signing
// same master key, multiple paths, multiple HD child keys, multiple messages
use std::iter::zip;
use std::{fs, time};

use itertools::multiunzip;

use curv::{
    arithmetic::{BasicOps, Converter, Modulo},
    cryptographic_primitives::{
        proofs::{sigma_correct_homomorphic_elgamal_enc::HomoELGamalProof, sigma_dlog::DLogProof},
        secret_sharing::feldman_vss::VerifiableSS,
    },
    elliptic::curves::{secp256_k1::Secp256k1, Point, Scalar},
    BigInt,
};
use multi_party_ecdsa::{protocols::multi_party_ecdsa::gg_2018::party_i::*, utilities::mta::*};
use paillier::*;
use reqwest::blocking::Client;
use serde_json::{json, Value};
use sha2::Sha256;

use crate::common::{
    broadcast, check_sig, poll_for_broadcasts, poll_for_p2p, sendp2p, Params, ParamsG, PartySignup,
};

pub fn run_sign_batch(
    addr: &String,
    party_keys: Keys,
    shared_keys: SharedKeys,
    party_id: u16,
    vss_scheme_vec: &mut Vec<VerifiableSS<Secp256k1>>,
    paillier_key_vector: Vec<EncryptionKey>,
    y_sum_batch: &Vec<Point<Secp256k1>>,
    params: &ParamsG,
    message_str_batch: &Vec<&str>,
    tweak_sk_batch: &Vec<Scalar<Secp256k1>>,
) -> Value {
    let client = Client::new();
    let delay = time::Duration::from_millis(25);
    let threshold: u16 = params.threshold.parse::<u16>().unwrap();
    let parties: u16 = params.parties.parse::<u16>().unwrap();
    let share_count: u16 = params.share_count.parse::<u16>().unwrap();
    println!(
        "threshold: {}, parties: {}, share count: {}",
        threshold, parties, share_count
    );
    assert!(parties > threshold, "PARTIES smaller than THRESHOLD + 1");
    assert!(parties < share_count + 1, "PARTIES bigger than SHARE_COUNT");

    assert!(
        y_sum_batch.len() == message_str_batch.len(),
        "pk/msg LENGTH not match!"
    );
    assert!(
        y_sum_batch.len() == tweak_sk_batch.len(),
        "pk/tweak LENGTH not match!"
    );
    let batch: usize = y_sum_batch.len();

    // Signup
    let (party_num_int, uuid) = match signup(&addr, &client, &params).unwrap() {
        PartySignup { number, uuid } => (number, uuid),
    };

    let debug = json!({"manager_addr": &addr, "party_num": party_num_int, "uuid": uuid});
    println!("{}", serde_json::to_string_pretty(&debug).unwrap());

    // round 0: collect signer IDs
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round0",
        serde_json::to_string(&party_id).unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round0_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round0",
        uuid.clone(),
    );
    let mut signers_vec = round0_ans_vec
        .iter()
        .map(|m| serde_json::from_str::<u16>(m).unwrap() - 1)
        .collect::<Vec<_>>();
    signers_vec.insert(party_num_int as usize - 1, party_id - 1);

    /*
    // to be theoretically correct everywhere
    if sign_at_path == true {
        // update uj * G as (uj + tweak_sk) * G
        // where j = party_id of party_num_int == 1
        vss_scheme_vec[usize::from(signers_vec[usize::from(0u16)])].commitments[0] =
            vss_scheme_vec[usize::from(signers_vec[usize::from(0u16)])].commitments[0].clone()
            + Point::generator() * tweak_sk;
    }
    let mut private = PartyPrivate::set_private(party_keys.clone(), shared_keys);
    if sign_at_path == true {
        if party_num_int == 1 {
            // update uj as (uj + tweak_sk) and xj as (xj + tweak_sk)
            private = private.update_private_key(&tweak_sk, &tweak_sk);
        } else {
            // only update xi as (xi + tweak_sk)
            private = private.update_private_key(&Scalar::<Secp256k1>::zero(), &tweak_sk);
        }
    }
    */

    // to be practically tricky, only applicable to sign
    // (1) ignore sign_at_path
    // (2) omit updates for all ui
    // (3) only update u1 * G as (u1 + tweak_sk) * G and all xi as (xi + tweak_sk)
    let mut vss_scheme_vec_batch: Vec<Vec<VerifiableSS<Secp256k1>>> = Vec::new();
    let mut sign_keys_batch: Vec<SignKeys> = Vec::new();

    for tweak_sk in tweak_sk_batch.iter() {
        let mut vss_scheme_vec_copy = vss_scheme_vec.clone();
        vss_scheme_vec_copy[0].commitments[0] =
            &vss_scheme_vec_copy[0].commitments[0] + Point::generator() * tweak_sk;
        let mut private = PartyPrivate::set_private(party_keys.clone(), shared_keys.clone());
        private = private.update_private_key(&Scalar::<Secp256k1>::zero(), tweak_sk);
        let sign_keys = SignKeys::create(
            &private,
            &vss_scheme_vec[usize::from(signers_vec[usize::from(party_num_int - 1)])],
            signers_vec[usize::from(party_num_int - 1)],
            &signers_vec,
        );
        vss_scheme_vec_batch.push(vss_scheme_vec_copy);
        sign_keys_batch.push(sign_keys);
    }

    let (com_batch, decommit_batch): (Vec<_>, Vec<_>) = sign_keys_batch
        .iter()
        .map(|x| x.phase1_broadcast())
        .collect::<Vec<(SignBroadcastPhase1, SignDecommitPhase1)>>()
        .iter()
        .cloned()
        .unzip();
    let (m_a_k_batch, _): (Vec<_>, Vec<_>) = sign_keys_batch
        .iter()
        .map(|x| MessageA::a(&x.k_i, &party_keys.ek, &[]))
        .collect::<Vec<(MessageA, BigInt)>>()
        .iter()
        .cloned()
        .unzip();

    // round 1: send commitment and do MtA/MtAwc (a) (b)
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round1",
        serde_json::to_string(&(com_batch.clone(), m_a_k_batch.clone())).unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round1_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round1",
        uuid.clone(),
    );

    let (mut bc1_batch_vec, m_a_batch_vec): (Vec<_>, Vec<_>) = round1_ans_vec
        .iter()
        .map(|m| serde_json::from_str::<(Vec<SignBroadcastPhase1>, Vec<MessageA>)>(m).unwrap())
        .collect::<Vec<_>>()
        .iter()
        .cloned()
        .unzip(); // len parties, each element of len batch
    bc1_batch_vec.insert(party_num_int as usize - 1, com_batch);

    // transpose
    let mut bc1_vec_batch: Vec<Vec<SignBroadcastPhase1>> = transpose(bc1_batch_vec);
    let m_a_vec_batch: Vec<Vec<MessageA>> = transpose(m_a_batch_vec);

    let mut m_b_gamma_send_vec_batch: Vec<Vec<MessageB>> = Vec::new();
    let mut beta_vec_batch: Vec<Vec<Scalar<Secp256k1>>> = Vec::new();
    let mut m_b_w_send_vec_batch: Vec<Vec<MessageB>> = Vec::new();
    let mut ni_vec_batch: Vec<Vec<Scalar<Secp256k1>>> = Vec::new();
    for (sign_keys, m_a_vec) in zip(&sign_keys_batch, &m_a_vec_batch) {
        // do MtA/MtAwc (c) (d)
        let mut m_b_gamma_send_vec: Vec<MessageB> = Vec::new();
        let mut beta_vec: Vec<Scalar<Secp256k1>> = Vec::new();
        let mut m_b_w_send_vec: Vec<MessageB> = Vec::new();
        let mut ni_vec: Vec<Scalar<Secp256k1>> = Vec::new();
        let mut j = 0;
        for i in 1..=parties {
            if i != party_num_int {
                let (m_b_gamma, beta_gamma, _, _) = MessageB::b(
                    &sign_keys.gamma_i,
                    &paillier_key_vector[usize::from(signers_vec[usize::from(i - 1)])],
                    m_a_vec[j].clone(),
                    &[],
                )
                .unwrap();
                let (m_b_w, beta_wi, _, _) = MessageB::b(
                    &sign_keys.w_i,
                    &paillier_key_vector[usize::from(signers_vec[usize::from(i - 1)])],
                    m_a_vec[j].clone(),
                    &[],
                )
                .unwrap();
                m_b_gamma_send_vec.push(m_b_gamma);
                m_b_w_send_vec.push(m_b_w);
                beta_vec.push(beta_gamma);
                ni_vec.push(beta_wi);
                j = j + 1;
            }
        }
        m_b_gamma_send_vec_batch.push(m_b_gamma_send_vec);
        m_b_w_send_vec_batch.push(m_b_w_send_vec);
        beta_vec_batch.push(beta_vec);
        ni_vec_batch.push(ni_vec);
    }

    // (backward) transpose
    let m_b_gamma_send_batch_vec: Vec<Vec<MessageB>> = transpose(m_b_gamma_send_vec_batch);
    let m_b_w_send_batch_vec: Vec<Vec<MessageB>> = transpose(m_b_w_send_vec_batch);

    // round 2: send Paillier ciphertext
    let mut j = 0;
    for i in 1..=parties {
        if i != party_num_int {
            assert!(sendp2p(
                &addr,
                &client,
                party_num_int,
                i,
                "round2",
                serde_json::to_string(&(
                    m_b_gamma_send_batch_vec[j].clone(),
                    m_b_w_send_batch_vec[j].clone(),
                ))
                .unwrap(),
                uuid.clone(),
            )
            .is_ok());
            j = j + 1;
        }
    }

    let round2_ans_vec = poll_for_p2p(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round2",
        uuid.clone(),
    );

    let (m_b_gamma_rec_batch_vec, m_b_w_rec_batch_vec): (Vec<Vec<MessageB>>, Vec<Vec<MessageB>>) =
        round2_ans_vec
            .iter()
            .map(|m| serde_json::from_str::<(Vec<MessageB>, Vec<MessageB>)>(m).unwrap())
            .collect::<Vec<_>>()
            .iter()
            .cloned()
            .unzip(); // len parties - 1, len parties - 1

    // transpose
    let m_b_gamma_rec_vec_batch: Vec<Vec<MessageB>> = transpose(m_b_gamma_rec_batch_vec);
    let m_b_w_rec_vec_batch: Vec<Vec<MessageB>> = transpose(m_b_w_rec_batch_vec);

    let mut alpha_vec_batch: Vec<Vec<Scalar<Secp256k1>>> = Vec::new();
    let mut miu_vec_batch: Vec<Vec<Scalar<Secp256k1>>> = Vec::new();

    for (((m_b_gamma_rec_vec, m_b_w_rec_vec), vss_scheme_vec), sign_keys) in zip(
        zip(
            zip(&m_b_gamma_rec_vec_batch, &m_b_w_rec_vec_batch),
            &vss_scheme_vec_batch,
        ),
        &sign_keys_batch,
    ) {
        // do MtA (e) / MtAwc (e) (f)
        let mut alpha_vec: Vec<Scalar<Secp256k1>> = Vec::new();
        let mut miu_vec: Vec<Scalar<Secp256k1>> = Vec::new();

        let xi_com_vec = Keys::get_commitments_to_xi(&vss_scheme_vec);
        let mut j = 0;
        for i in 1..=parties {
            if i != party_num_int {
                let m_b = m_b_gamma_rec_vec[j].clone();
                let alpha_ij_gamma = m_b
                    .verify_proofs_get_alpha(&party_keys.dk, &sign_keys.k_i)
                    .expect("wrong dlog or m_b");
                let m_b = m_b_w_rec_vec[j].clone();
                let alpha_ij_wi = m_b
                    .verify_proofs_get_alpha(&party_keys.dk, &sign_keys.k_i)
                    .expect("wrong dlog or m_b");
                alpha_vec.push(alpha_ij_gamma.0);
                miu_vec.push(alpha_ij_wi.0);
                let g_w_i = Keys::update_commitments_to_xi(
                    &xi_com_vec[usize::from(signers_vec[usize::from(i - 1)])],
                    &vss_scheme_vec[usize::from(signers_vec[usize::from(i - 1)])],
                    signers_vec[usize::from(i - 1)],
                    &signers_vec,
                );
                assert_eq!(m_b.b_proof.pk.clone(), g_w_i);
                j = j + 1;
            }
        }
        alpha_vec_batch.push(alpha_vec);
        miu_vec_batch.push(miu_vec);
    }

    let delta_i_batch: Vec<Scalar<Secp256k1>> =
        zip(zip(&sign_keys_batch, &alpha_vec_batch), &beta_vec_batch)
            .map(|((x, y), z)| x.phase2_delta_i(y, z))
            .collect();
    let sigma_batch: Vec<Scalar<Secp256k1>> =
        zip(zip(&sign_keys_batch, &miu_vec_batch), &ni_vec_batch)
            .map(|((x, y), z)| x.phase2_sigma_i(y, z))
            .collect();

    // round 3: send delta_i
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round3",
        serde_json::to_string(&delta_i_batch).unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round3_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round3",
        uuid.clone(),
    );
    let mut delta_batch_vec: Vec<Vec<Scalar<Secp256k1>>> = Vec::new();
    format_vec_from_reads(
        &round3_ans_vec,
        party_num_int as usize,
        delta_i_batch,
        &mut delta_batch_vec,
    );

    // transpose
    let delta_vec_batch: Vec<Vec<Scalar<Secp256k1>>> = transpose(delta_batch_vec);

    let delta_inv_batch: Vec<Scalar<Secp256k1>> = delta_vec_batch
        .iter()
        .map(|x| SignKeys::phase3_reconstruct_delta(x))
        .collect();

    // round 4: send decommitment to g_gamma_i
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round4",
        serde_json::to_string(&decommit_batch).unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round4_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round4",
        uuid.clone(),
    );
    let mut decommit_batch_vec: Vec<Vec<SignDecommitPhase1>> = Vec::new();
    format_vec_from_reads(
        &round4_ans_vec,
        party_num_int as usize,
        decommit_batch,
        &mut decommit_batch_vec,
    );

    let decomm_i_batch = decommit_batch_vec.remove((party_num_int - 1) as usize);
    for bc1 in bc1_vec_batch.iter_mut() {
        bc1.remove((party_num_int - 1) as usize);
    }

    let b_proof_vec_batch = m_b_gamma_rec_vec_batch
        .iter()
        .map(|m_b_gamma_rec_vec| {
            (0..m_b_gamma_rec_vec.len())
                .map(|i| &m_b_gamma_rec_vec[i].b_proof)
                .collect::<Vec<&DLogProof<Secp256k1, Sha256>>>()
        })
        .collect::<Vec<_>>();

    // transpose
    let decommit_vec_batch: Vec<Vec<SignDecommitPhase1>> = transpose(decommit_batch_vec);

    let mut R_batch: Vec<Point<Secp256k1>> = Vec::new();
    for ((((delta_inv, b_proof_vec), decommit_vec), bc1_vec), decommit_i) in zip(
        zip(
            zip(
                zip(&delta_inv_batch, &b_proof_vec_batch),
                &decommit_vec_batch,
            ),
            &bc1_vec_batch,
        ),
        &decomm_i_batch,
    ) {
        let R = SignKeys::phase4(&delta_inv, &b_proof_vec, decommit_vec.to_vec(), &bc1_vec)
            .expect("bad gamma_i decommit");
        // add local g_gamma_i
        let R = R + (decommit_i.g_gamma_i).clone() * delta_inv;
        R_batch.push(R);
    }

    let mut message_bn_batch: Vec<BigInt> = Vec::new();
    let mut message_int_batch: Vec<BigInt> = Vec::new();
    let mut local_sig_batch: Vec<LocalSignature> = Vec::new();
    for ((((message_str, sign_keys), sigma), y_sum), R) in zip(
        zip(
            zip(zip(message_str_batch, &sign_keys_batch), &sigma_batch),
            y_sum_batch,
        ),
        &R_batch,
    ) {
        let message = match hex::decode(message_str.clone()) {
            Ok(x) => x,
            Err(_e) => message_str.as_bytes().to_vec(),
        };
        let message = &message[..];

        // assume the message is already hashed (by the signer)
        let message_bn = BigInt::from_bytes(message);
        let message_int = BigInt::from_bytes(message);
        let two = BigInt::from(2);
        let message_bn = message_bn.modulus(&two.pow(256));
        let local_sig =
            LocalSignature::phase5_local_sig(&sign_keys.k_i, &message_bn, R, sigma, y_sum);
        message_bn_batch.push(message_bn);
        message_int_batch.push(message_int);
        local_sig_batch.push(local_sig);
    }

    let (phase5_com_batch, phase_5a_decom_batch, helgamal_proof_batch, dlog_proof_rho_batch): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = multiunzip(
        local_sig_batch
            .iter()
            .map(|local_sig| local_sig.phase5a_broadcast_5b_zkproof())
            .collect::<Vec<(
                Phase5Com1,
                Phase5ADecom1,
                HomoELGamalProof<Secp256k1, Sha256>,
                DLogProof<Secp256k1, Sha256>,
            )>>(),
    );

    // round 5: GG18 Phase(5A)
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round5",
        serde_json::to_string(&phase5_com_batch).unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round5_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round5",
        uuid.clone(),
    );
    let mut commit5a_batch_vec: Vec<Vec<Phase5Com1>> = Vec::new();
    format_vec_from_reads(
        &round5_ans_vec,
        party_num_int.clone() as usize,
        phase5_com_batch,
        &mut commit5a_batch_vec,
    );

    // round 6: GG18 Phase(5B)
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round6",
        serde_json::to_string(&(
            phase_5a_decom_batch.clone(),
            helgamal_proof_batch.clone(),
            dlog_proof_rho_batch.clone()
        ))
        .unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round6_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round6",
        uuid.clone(),
    );
    let mut decommit5a_and_elgamal_and_dlog_batch_vec: Vec<(
        Vec<Phase5ADecom1>,
        Vec<HomoELGamalProof<Secp256k1, Sha256>>,
        Vec<DLogProof<Secp256k1, Sha256>>,
    )> = Vec::new();
    format_vec_from_reads(
        &round6_ans_vec,
        party_num_int as usize,
        (
            phase_5a_decom_batch.clone(),
            helgamal_proof_batch.clone(),
            dlog_proof_rho_batch.clone(),
        ),
        &mut decommit5a_and_elgamal_and_dlog_batch_vec,
    );

    let decommit5a_and_elgamal_batch_vec_includes_i =
        decommit5a_and_elgamal_and_dlog_batch_vec.clone();

    decommit5a_and_elgamal_and_dlog_batch_vec.remove((party_num_int - 1) as usize); // excl. party i

    // tuple-matrix transpose
    for tuple in &mut decommit5a_and_elgamal_and_dlog_batch_vec {
        tuple.0.reverse();
        tuple.1.reverse();
        tuple.2.reverse();
    }
    let decommit5a_and_elgamal_and_dlog_vec_batch: Vec<
        Vec<(
            Phase5ADecom1,
            HomoELGamalProof<Secp256k1, Sha256>,
            DLogProof<Secp256k1, Sha256>,
        )>,
    > = (0..batch)
        .map(|_| {
            decommit5a_and_elgamal_and_dlog_batch_vec
                .iter_mut()
                .map(|(x, y, z)| (x.pop().unwrap(), y.pop().unwrap(), z.pop().unwrap()))
                .collect::<Vec<(_, _, _)>>()
        })
        .collect();
    /*
    let decommit5a_and_elgamal_and_dlog_vec_batch: Vec<
        Vec<(
            Phase5ADecom1,
            HomoELGamalProof<Secp256k1, Sha256>,
            DLogProof<Secp256k1, Sha256>,
        )>,
    > = (0..batch)
        .map(|i| {
            decommit5a_and_elgamal_and_dlog_batch_vec
                .iter()
                .map(|(x, y, z)| {
                    (
                        (*x.iter().skip(i).next().unwrap()).clone(),
                        (*y.iter().skip(i).next().unwrap()).clone(),
                        (*z.iter().skip(i).next().unwrap()).clone(),
                    )
                })
                .collect::<Vec<(_, _, _)>>()
        })
        .collect::<Vec<_>>();
    */

    commit5a_batch_vec.remove((party_num_int - 1) as usize); // excl. party i

    // transpose
    let commit5a_vec_batch: Vec<Vec<Phase5Com1>> = transpose(commit5a_batch_vec);

    let mut phase5_com2_batch: Vec<Phase5Com2> = Vec::new();
    let mut phase_5d_decom2_batch: Vec<Phase5DDecom2> = Vec::new();
    for ((((decommit5a_and_elgamal_and_dlog_vec, commit5a_vec), phase_5a_decom), R), local_sig) in
        zip(
            zip(
                zip(
                    zip(
                        &decommit5a_and_elgamal_and_dlog_vec_batch,
                        &commit5a_vec_batch,
                    ),
                    &phase_5a_decom_batch,
                ),
                R_batch,
            ),
            &local_sig_batch,
        )
    {
        let phase_5a_decomm_vec = (0..parties - 1)
            .map(|i| decommit5a_and_elgamal_and_dlog_vec[i as usize].0.clone())
            .collect::<Vec<Phase5ADecom1>>();
        let phase_5a_elgamal_vec = (0..parties - 1)
            .map(|i| decommit5a_and_elgamal_and_dlog_vec[i as usize].1.clone())
            .collect::<Vec<HomoELGamalProof<Secp256k1, Sha256>>>();
        let phase_5a_dlog_vec = (0..parties - 1)
            .map(|i| decommit5a_and_elgamal_and_dlog_vec[i as usize].2.clone())
            .collect::<Vec<DLogProof<Secp256k1, Sha256>>>();
        let (phase5_com2, phase_5d_decom2) = local_sig
            .phase5c(
                &phase_5a_decomm_vec,
                &commit5a_vec,
                &phase_5a_elgamal_vec,
                &phase_5a_dlog_vec,
                &phase_5a_decom.V_i,
                &R.clone(),
            )
            .expect("error phase5");
        phase5_com2_batch.push(phase5_com2);
        phase_5d_decom2_batch.push(phase_5d_decom2);
    }

    // round 7: GG18 Phase(5C)
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round7",
        serde_json::to_string(&phase5_com2_batch).unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round7_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round7",
        uuid.clone(),
    );
    let mut commit5c_batch_vec: Vec<Vec<Phase5Com2>> = Vec::new();
    format_vec_from_reads(
        &round7_ans_vec,
        party_num_int.clone() as usize,
        phase5_com2_batch,
        &mut commit5c_batch_vec,
    );

    // round 8: GG18 Phase(5D)
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round8",
        serde_json::to_string(&phase_5d_decom2_batch).unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round8_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round8",
        uuid.clone(),
    );
    let mut decommit5d_batch_vec: Vec<Vec<Phase5DDecom2>> = Vec::new();
    format_vec_from_reads(
        &round8_ans_vec,
        party_num_int.clone() as usize,
        phase_5d_decom2_batch.clone(),
        &mut decommit5d_batch_vec,
    );

    let phase_5a_decomm_batch_vec_includes_i = (0..parties)
        .map(|i| {
            decommit5a_and_elgamal_batch_vec_includes_i[i as usize]
                .0
                .clone()
        })
        .collect::<Vec<Vec<Phase5ADecom1>>>();

    // transpose
    let commit5c_vec_batch: Vec<Vec<Phase5Com2>> = transpose(commit5c_batch_vec);
    let decommit5d_vec_batch: Vec<Vec<Phase5DDecom2>> = transpose(decommit5d_batch_vec);
    let phase_5a_decomm_vec_batch_includes_i: Vec<Vec<Phase5ADecom1>> =
        transpose(phase_5a_decomm_batch_vec_includes_i);

    let mut s_i_batch: Vec<Scalar<Secp256k1>> = Vec::new();
    for (((local_sig, decommit5d_vec), commit5c_vec), phase_5a_decomm_vec_includes_i) in zip(
        zip(
            zip(&local_sig_batch, &decommit5d_vec_batch),
            &commit5c_vec_batch,
        ),
        &phase_5a_decomm_vec_batch_includes_i,
    ) {
        let s_i = local_sig
            .phase5d(
                &decommit5d_vec,
                &commit5c_vec,
                &phase_5a_decomm_vec_includes_i,
            )
            .expect("bad com 5d");
        s_i_batch.push(s_i);
    }

    // round 9: GG18 Phase(5E)
    assert!(broadcast(
        &addr,
        &client,
        party_num_int,
        "round9",
        serde_json::to_string(&s_i_batch).unwrap(),
        uuid.clone(),
    )
    .is_ok());
    let round9_ans_vec = poll_for_broadcasts(
        &addr,
        &client,
        party_num_int,
        parties,
        delay,
        "round9",
        uuid.clone(),
    );
    let mut s_i_batch_vec: Vec<Vec<Scalar<Secp256k1>>> = Vec::new();
    format_vec_from_reads(
        &round9_ans_vec,
        party_num_int.clone() as usize,
        s_i_batch,
        &mut s_i_batch_vec,
    );

    s_i_batch_vec.remove((party_num_int - 1) as usize); // excl. party i

    // transpose
    let s_i_vec_batch: Vec<Vec<Scalar<Secp256k1>>> = transpose(s_i_batch_vec);

    let sig_batch = zip(&local_sig_batch, &s_i_vec_batch)
        .map(|(local_sig, s_i_vec)| {
            local_sig
                .output_signature(&s_i_vec)
                .expect("verification failed")
        })
        .collect::<Vec<SignatureRecid>>();

    let mut retall = json!({});

    let mut j = 1;
    for (((y_sum, sig), message_int), message_bn) in zip(
        zip(zip(y_sum_batch, &sig_batch), &message_int_batch),
        &message_bn_batch,
    ) {
        // println!("child pubkey {:?}: {:#?} \n", j, y_sum);
        // println!("verifying signature with child pubkey {:?}...", j);

        let ret_dict = json!({
            "r": BigInt::from_bytes(sig.r.to_bytes().as_ref()).to_str_radix(16),
            "s": BigInt::from_bytes(sig.s.to_bytes().as_ref()).to_str_radix(16),
            "status": "signature_ready",
            "recid": sig.recid.clone(),
            "x": &y_sum.x_coord(),
            "y": &y_sum.y_coord(),
            "msg_int": message_int,
        });
        check_sig(&sig.r, &sig.s, &message_bn, &y_sum);
        println!("signature passed verification with child pubkey {:?}", j);

        retall
            .as_object_mut()
            .unwrap()
            .insert("signature".to_string() + &j.to_string(), ret_dict);
        // fs::write(
        //     "signature".to_string() + &j.to_string(),
        //     ret_dict.to_string(),
        // )
        // .expect("Unable to save!");
        // println!("party {:?} Output Signature {:?}: \n", party_num_int, j);
        // println!("r: {:#?}", sig.r);
        // println!("s: {:#?} \n", sig.s);
        // println!("recid: {:?} \n", sig.recid.clone());
        // println!("x: {:#?}", &y_sum.x_coord());
        // println!("y: {:#?}", &y_sum.y_coord());
        // println!("msg_int: {}", message_int);
        j = j + 1;
    }
    retall
}

fn format_vec_from_reads<'a, T: serde::Deserialize<'a> + Clone>(
    ans_vec: &'a Vec<String>,
    party_num: usize,
    value_i: T,
    new_vec: &'a mut Vec<T>,
) {
    let mut j = 0;
    for i in 1..ans_vec.len() + 2 {
        if i == party_num {
            new_vec.push(value_i.clone());
        } else {
            let value_j: T = serde_json::from_str(&ans_vec[j]).unwrap();
            new_vec.push(value_j);
            j = j + 1;
        }
    }
}

pub fn postb<T>(addr: &String, client: &Client, path: &str, body: T) -> Option<String>
where
    T: serde::ser::Serialize,
{
    let res = client
        .post(&format!("{}/{}", addr, path))
        .json(&body)
        .send();
    Some(res.unwrap().text().unwrap())
}

pub fn signup(addr: &String, client: &Client, params: &ParamsG) -> Result<PartySignup, ()> {
    let res_body = postb(&addr, &client, "signupsign", params).unwrap();
    let answer: Result<PartySignup, ()> = serde_json::from_str(&res_body).unwrap();
    return answer;
}

/*
fn transpose<T>(mat: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!mat.is_empty());
    let num_cols = mat.first().unwrap().len();
    let mut row: Vec<_> = mat.into_iter().map(Vec::into_iter).collect();
    (0..num_cols)
        .map(|_| row
            .iter_mut()
            .map(|x| x.next().unwrap())
            .collect::<Vec<T>>())
        .collect()
}
*/

fn transpose<T>(mut mat: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!mat.is_empty());
    for row in &mut mat {
        row.reverse();
    }
    (0..mat[0].len())
        .map(|_| {
            mat.iter_mut()
                .map(|row| row.pop().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

/*
// This one not working yet!
fn transpose<T>(mat: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!mat.is_empty());
    let mut mat_transposed = (0..mat[0].len()).map(|_| vec![]).collect::<Vec<_>>();
    for row in mat {
        for (x, row_transposed) in row.into_iter().zip(&mut mat_transposed) {
            row.push(x);
        }
    }
    mat_transposed
}
*/