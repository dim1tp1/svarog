use std::collections::HashMap;
use std::ops::Deref;

use curv::{arithmetic::Converter, BigInt};
use curv::{
    cryptographic_primitives::secret_sharing::feldman_vss::{ShamirSecretSharing, VerifiableSS},
    elliptic::curves::{secp256_k1::Secp256k1, Point, Scalar},
};
use lx_grpc::gg18 as GG18Proto;
use lx_grpc::prelude::prost::Message;
use lx_grpc::prelude::tokio;
use lx_grpc::prelude::tokio::io::{AsyncReadExt, AsyncWriteExt};
pub use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2018::party_i::{
    Keys as GG18Keys, SharedKeys as GG18SharedKeys,
};
use paillier::{DecryptionKey, EncryptionKey};

use serde::{Deserialize, Serialize};
use xuanmi_base_support::*;

use crate::Batch;
pub type Vss = VerifiableSS<Secp256k1>;

#[derive(Clone, Serialize, Deserialize)]
pub struct RuntimeKeyStore {
    pub chain_code: [u8; 32],
    pub party_keys: GG18Keys,
    pub shared_keys: GG18SharedKeys,
    pub batch_vss_scheme: Batch<Vss>,
    pub batch_paillier_key: Batch<EncryptionKey>,
    pub root_pubkey: Point<Secp256k1>,

    pub owner_id: String,
    pub key_id: String,
}

impl RuntimeKeyStore {
    /// Convert to protobuf message
    pub fn marshall_proto(&self) -> GG18Proto::KeyStore {
        GG18Proto::KeyStore {
            chain_code: self.chain_code.to_vec(),
            root_pubkey: self.root_pubkey.to_bytes(true).deref().to_vec(),
            party_keys: GG18Proto::Gg18Keys {
                u_i: self.party_keys.u_i.to_bigint().to_bytes(),
                y_i: self.party_keys.y_i.to_bytes(true).deref().to_vec(),
                dk: GG18Proto::DecryptionKey {
                    p: self.party_keys.dk.p.to_bytes(),
                    q: self.party_keys.dk.q.to_bytes(),
                },
                ek: GG18Proto::EncryptionKey {
                    n: self.party_keys.ek.n.to_bytes(),
                    nn: self.party_keys.ek.nn.to_bytes(),
                },
                party_index: self.party_keys.party_index as u64,
            },
            shared_keys: GG18Proto::Gg18SharedKeys {
                y: self.shared_keys.y.to_bytes(true).deref().to_vec(),
                x_i: self.shared_keys.x_i.to_bigint().to_bytes(),
            },
            batch_vss_scheme: {
                let mut batch_vss_scheme: HashMap<u64, GG18Proto::Vss> = HashMap::with_capacity(16);
                for (party_id, vss) in self.batch_vss_scheme.iter() {
                    let vss_proto = GG18Proto::Vss {
                        parameters: GG18Proto::ShamirSecretSharing {
                            threshold: vss.parameters.threshold as u64,
                            share_count: vss.parameters.share_count as u64,
                        },
                        commitments: {
                            let mut commitments: Vec<Vec<u8>> = Vec::with_capacity(16);
                            for commitment in vss.commitments.iter() {
                                let com = commitment.to_bytes(true).deref().to_vec();
                                commitments.push(com);
                            }
                            commitments
                        },
                    };
                    let _ = batch_vss_scheme.insert(*party_id as u64, vss_proto);
                }
                batch_vss_scheme
            },
            batch_paillier_key: {
                let mut batch_paillier_key: HashMap<u64, GG18Proto::EncryptionKey> =
                    HashMap::with_capacity(16);
                for (party_id, paillier_key) in self.batch_paillier_key.iter() {
                    let paillier_key_proto = GG18Proto::EncryptionKey {
                        n: paillier_key.n.to_bytes(),
                        nn: paillier_key.nn.to_bytes(),
                    };
                    let _ = batch_paillier_key.insert(*party_id as u64, paillier_key_proto);
                }
                batch_paillier_key
            },
            owner_id: self.owner_id.clone(),
            key_id: self.key_id.clone(),
        }
    }

    /// Convert from protobuf message
    pub fn unmarshall_proto(proto: &GG18Proto::KeyStore) -> Outcome<Self> {
        Ok(Self {
            chain_code: proto.chain_code.as_slice().try_into().catch_()?,
            root_pubkey: Point::<Secp256k1>::from_bytes(&proto.root_pubkey).catch_()?,
            party_keys: GG18Keys {
                u_i: Scalar::<Secp256k1>::from_bigint(&BigInt::from_bytes(&proto.party_keys.u_i)),
                y_i: Point::<Secp256k1>::from_bytes(&proto.party_keys.y_i).catch_()?,
                dk: DecryptionKey {
                    p: BigInt::from_bytes(&proto.party_keys.dk.p),
                    q: BigInt::from_bytes(&proto.party_keys.dk.q),
                },
                ek: EncryptionKey {
                    n: BigInt::from_bytes(&proto.party_keys.ek.n),
                    nn: BigInt::from_bytes(&proto.party_keys.ek.nn),
                },
                party_index: proto.party_keys.party_index as u16,
            },
            shared_keys: GG18SharedKeys {
                y: Point::<Secp256k1>::from_bytes(&proto.shared_keys.y).catch_()?,
                x_i: Scalar::<Secp256k1>::from_bigint(&BigInt::from_bytes(&proto.shared_keys.x_i)),
            },
            batch_vss_scheme: {
                let mut batch_vss_scheme: HashMap<usize, Vss> = HashMap::with_capacity(16);
                for (party_id, vss_proto) in proto.batch_vss_scheme.iter() {
                    let vss = Vss {
                        parameters: ShamirSecretSharing {
                            threshold: vss_proto.parameters.threshold as u16,
                            share_count: vss_proto.parameters.share_count as u16,
                        },
                        commitments: {
                            let mut commitments: Vec<Point<Secp256k1>> = Vec::with_capacity(16);
                            for commitment in vss_proto.commitments.iter() {
                                let com = Point::<Secp256k1>::from_bytes(commitment).catch_()?;
                                commitments.push(com);
                            }
                            commitments
                        },
                    };
                    let _ = batch_vss_scheme.insert(*party_id as usize, vss);
                }
                batch_vss_scheme
            },
            batch_paillier_key: {
                let mut batch_paillier_key: HashMap<usize, EncryptionKey> =
                    HashMap::with_capacity(16);
                for (party_id, paillier_key_proto) in proto.batch_paillier_key.iter() {
                    let paillier_key = EncryptionKey {
                        n: BigInt::from_bytes(&paillier_key_proto.n),
                        nn: BigInt::from_bytes(&paillier_key_proto.nn),
                    };
                    let _ = batch_paillier_key.insert(*party_id as usize, paillier_key);
                }
                batch_paillier_key
            },
            owner_id: proto.owner_id.clone(),
            key_id: proto.key_id.clone(),
        })
    }

    pub fn marshall_bytes(&self) -> Outcome<Vec<u8>> {
        let proto = self.marshall_proto();
        let mut buf: Vec<u8> = Vec::new();
        proto.encode(&mut buf).catch_()?;
        Ok(buf)
    }

    pub fn unmarshall_bytes(buf: &[u8]) -> Outcome<Self> {
        let proto = GG18Proto::KeyStore::decode(buf).catch_()?;
        let proto = Self::unmarshall_proto(&proto).catch_()?;
        Ok(proto)
    }

    pub async fn marshall_proto_file(&self, path: &str) -> Outcome<()> {
        let proto = self.marshall_proto();
        let mut fd = tokio::fs::File::create(&path).await.catch_()?;
        fd.write_all(&proto.encode_to_vec()).await.catch_()?;
        Ok(())
    }

    pub async fn unmarshall_proto_file(path: &str) -> Outcome<Self> {
        let mut fd = tokio::fs::File::open(&path).await.catch_()?;
        let mut buf = Vec::new();
        fd.read_to_end(&mut buf).await.catch_()?;
        let proto = GG18Proto::KeyStore::decode(buf.as_slice()).catch_()?;

        let proto = Self::unmarshall_proto(&proto).catch_()?;
        Ok(proto)
    }

    pub async fn marshall_json_file(&self, path: &str) -> Outcome<()> {
        let buf = serde_json::to_string(self).catch_()?;
        let mut fd = tokio::fs::File::create(&path).await.catch_()?;
        fd.write_all(buf.as_bytes()).await.catch_()?;
        Ok(())
    }

    pub async fn unmarshall_json_file(path: &str) -> Outcome<Self> {
        let mut fd = tokio::fs::File::open(&path).await.catch_()?;
        let mut buf = String::new();
        fd.read_to_string(&mut buf).await.catch_()?;
        let keystore = serde_json::from_str(&buf).catch_()?;
        Ok(keystore)
    }

    pub fn party_id(&self) -> usize {
        self.party_keys.party_index as usize
    }
}
