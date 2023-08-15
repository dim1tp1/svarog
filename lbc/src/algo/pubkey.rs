use curv::elliptic::curves::{secp256_k1::Secp256k1, Point};

use super::hd;
use crate::RuntimeKeyStore;
use xuanmi_base_support::*;

pub type Pk = Point<Secp256k1>;

/// Fetch and compute the derived public key from root pubkey and chaincode.
pub fn algo_pubkey(keystore: &RuntimeKeyStore, derive: &str) -> Outcome<Pk> {
    match derive.is_empty() {
        true => Ok(keystore.root_pubkey.clone()),
        false => match hd::algo_get_hd_key(derive, &keystore.root_pubkey, &keystore.chain_code) {
            Ok((_, y_sum)) => Ok(y_sum),
            Err(e) => Err(e),
        },
    }
}

pub fn algo_pubkey_frompub(y_sum: Pk, chain_code: &[u8; 32], derive: &str) -> Outcome<Pk> {
    match derive.is_empty() {
        true => Ok(y_sum),
        false => match hd::algo_get_hd_key(derive, &y_sum, chain_code) {
            Ok((_, y_sum)) => Ok(y_sum),
            Err(e) => Err(e),
        },
    }
}
