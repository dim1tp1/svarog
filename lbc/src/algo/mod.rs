use std::collections::HashMap;
pub type Batch<T> = HashMap<usize, T>;

mod aes;
pub use aes::*;
mod keygen;
pub use keygen::*;
mod sign;
use lx_grpc::{
    lbcs::{mpc_progress::Harvest, MpcProgress},
    prelude::tokio::io::{AsyncReadExt, AsyncWriteExt},
};
pub use sign::*;
mod pubkey;
pub use pubkey::*;
mod hd;
pub use hd::*;

mod kms;
pub use kms::*;

mod keygen_root_mnem;
pub use keygen_root_mnem::*;

use lx_grpc::prelude::{tokio, tonic};
use xuanmi_base_support::*;

pub trait ToVecByKeyOrder<T> {
    fn values_sorted_by_key_asc(&self) -> Vec<T>;
    fn keys_asc(&self) -> Vec<usize>;
}

impl<T> ToVecByKeyOrder<T> for Batch<T>
where
    T: Clone,
{
    fn values_sorted_by_key_asc(&self) -> Vec<T> {
        // get all keys of a hashmap in ascending order
        // https://stackoverflow.com/questions/27582739/how-do-i-get-all-keys-of-a-hashmap-in-ascending-order
        let mut keys: Vec<usize> = self.keys().cloned().collect();
        keys.sort();
        let mut vals: Vec<T> = Vec::with_capacity(self.len());

        for key in keys {
            vals.push(self.get(&key).unwrap().clone())
        }
        vals
    }

    fn keys_asc(&self) -> Vec<usize> {
        let mut keys: Vec<usize> = self.keys().cloned().collect();
        keys.sort();
        keys
    }
}

pub trait MpcRound {
    fn increase(&mut self) -> &mut Self;
    fn harvest(&mut self, harvest: Harvest) -> &mut Self;
    fn message(&mut self, msg: impl AsRef<str>) -> &mut Self;
}

impl MpcRound for MpcProgress {
    fn increase(&mut self) -> &mut Self {
        self.current += 1;
        self
    }

    fn harvest(&mut self, harvest: Harvest) -> &mut Self {
        self.harvest = Some(harvest);
        self
    }

    fn message(&mut self, msg: impl AsRef<str>) -> &mut Self {
        self.message = msg.as_ref().to_owned();
        self
    }
}

#[tonic::async_trait]
pub trait CheckDirRwx {
    async fn check_dir_rwx(&self) -> Outcome<()>;
}

#[tonic::async_trait]
impl<STR> CheckDirRwx for STR
where
    STR: AsRef<str> + core::fmt::Display + Send + Sync,
{
    async fn check_dir_rwx(&self) -> Outcome<()> {
        let dir = self.as_ref();
        let metadata = tokio::fs::metadata(dir).await.catch_()?;
        assert_throw!(metadata.is_dir(), dir);

        // create (touch) a file in dir
        let filename = format!("{}/.check_dir_rwx", dir);
        let _ = tokio::fs::File::create(filename.clone()).await.catch(
            "PermissionDeniedException",
            &format!("Cannot create file in directory {}", dir),
        )?;

        return Ok(());
    }
}

async fn check_dir_rwx<STR>(dir: STR) -> Outcome<()>
where
    STR: AsRef<str> + core::fmt::Display + Send + Sync,
{
    let dir = dir.as_ref();
    let metadata = tokio::fs::metadata(dir).await.catch_()?;
    assert_throw!(metadata.is_dir(), dir);

    // list files in dir
    let _ = tokio::fs::read_dir(dir).await.catch(
        "PermissionDeniedException",
        &format!("No execute (i.e. search) permission on directory {}", dir),
    )?;

    // remove .check_dir_rwx
    let pid = std::process::id();
    let filename = format!("{}/.check_dir_rwx_pid{}", dir, pid);
    let _ = tokio::fs::remove_file(&filename).await;

    // write .check_dir_rwx
    let mut file = tokio::fs::File::create(&filename).await.catch(
        "PermissionDeniedException",
        &format!("Cannot create file in directory {}", dir),
    )?;
    file.write_all(b"Hello, world!").await.catch_()?;
    drop(file);

    // read .check_dir_rwx
    let mut file = tokio::fs::File::open(&filename).await.catch(
        "PermissionDeniedException",
        &format!("Cannot read file in directory {}", dir),
    )?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.catch_()?;
    assert_throw!(contents == "Hello, world!", dir);

    // remove .check_dir_rwx
    tokio::fs::remove_file(&filename).await.catch_()?;

    Ok(())
}
