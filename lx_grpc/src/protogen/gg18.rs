#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(bytes = "vec", required, tag = "1")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", required, tag = "2")]
    pub s: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, required, tag = "3")]
    pub v: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyStore {
    #[prost(bytes = "vec", required, tag = "1")]
    pub chain_code: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", required, tag = "2")]
    pub root_pubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, required, tag = "3")]
    pub party_keys: Gg18Keys,
    #[prost(message, required, tag = "4")]
    pub shared_keys: Gg18SharedKeys,
    #[prost(map = "uint64, message", tag = "5")]
    pub batch_vss_scheme: ::std::collections::HashMap<u64, Vss>,
    #[prost(map = "uint64, message", tag = "6")]
    pub batch_paillier_key: ::std::collections::HashMap<u64, EncryptionKey>,
    #[prost(string, required, tag = "14")]
    pub owner_id: ::prost::alloc::string::String,
    #[prost(string, required, tag = "15")]
    pub key_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gg18Keys {
    #[prost(bytes = "vec", required, tag = "1")]
    pub u_i: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", required, tag = "2")]
    pub y_i: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, required, tag = "3")]
    pub dk: DecryptionKey,
    #[prost(message, required, tag = "4")]
    pub ek: EncryptionKey,
    #[prost(uint64, required, tag = "5")]
    pub party_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptionKey {
    #[prost(bytes = "vec", required, tag = "1")]
    pub p: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", required, tag = "2")]
    pub q: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionKey {
    #[prost(bytes = "vec", required, tag = "1")]
    pub n: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", required, tag = "2")]
    pub nn: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gg18SharedKeys {
    #[prost(bytes = "vec", required, tag = "1")]
    pub y: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", required, tag = "2")]
    pub x_i: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vss {
    #[prost(message, required, tag = "1")]
    pub parameters: ShamirSecretSharing,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub commitments: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShamirSecretSharing {
    #[prost(uint64, required, tag = "1")]
    pub threshold: u64,
    #[prost(uint64, required, tag = "2")]
    pub share_count: u64,
}
