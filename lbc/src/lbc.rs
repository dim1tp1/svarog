#![allow(non_snake_case, non_upper_case_globals, dead_code)]
mod algo;
use std::pin::Pin;

pub use algo::*;
mod keystore;
mod lbm_client;
use keystore::RuntimeKeyStore;

use clap::{arg, Command};
use lbm_client::LubanManagerClient;
// One may be shocked by this name, but it's correct.
// The client is also a server dedicated for localhost client access.A
use lx_grpc::lbcs::luban_client_server::{LubanClient, LubanClientServer};
use lx_grpc::lbcs::*;
use lx_grpc::lbm::Void;
use lx_grpc::prelude::{tokio, tokio_stream, tonic};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{transport::Server, Request, Response, Status};
use xuanmi_base_support::*;
type MpcProgressStream = Pin<Box<dyn Stream<Item = Result<MpcProgress, Status>> + Send>>;

pub struct LubanClientService {
    lbm_server_url: String,
    keystore_dir: String,
}

impl LubanClientService {
    pub fn new(lbm_server_url: String, keystore_dir: String) -> Self {
        Self {
            lbm_server_url,
            keystore_dir,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("LubanClient")
        .about("Luban Client (LBC) Daemon.")
        .arg(arg!(-p --port <LOCAL_PORT>).required(true))
        .arg(arg!(-d --dir <KEYSTORE_DIR>).required(true))
        .arg(arg!(-s --server <LBM_SERVER_URL>).required(true))
        .arg(
            arg!(--log <LOG_LEVEL>)
                .required(false)
                .default_value("info"),
        )
        .get_matches();
    let p = matches.get_one::<String>("port").unwrap().to_owned();
    let hostport = format!("127.0.0.1:{}", &p);
    let d = matches.get_one::<String>("dir").unwrap().to_owned();
    let s = matches.get_one::<String>("server").unwrap().to_owned();
    let log_level = matches.get_one::<String>("log").unwrap().to_owned();
    init_tracer!("logs", format!("lbc@{}.log", &p), &log_level);

    tracing::info!(
        "Start lbclient with local_port={}, keystore_dir={}, lbm_server_url={}",
        p,
        d,
        s
    );
    let service = LubanClientService::new(s, d);
    println!("Luban Client Daemon is running on {}", &hostport);

    Server::builder()
        .add_service(LubanClientServer::new(service))
        .serve(hostport.parse().unwrap())
        .await?;
    Ok(())
}

#[tonic::async_trait]
impl LubanClient for LubanClientService {
    type ClientKeygenStream = MpcProgressStream;
    type ClientShareRootKeyStream = MpcProgressStream;
    type ClientRescueKeyStream = MpcProgressStream;
    type ClientSignStream = MpcProgressStream;

    async fn client_keygen(
        &self,
        request: Request<BasicMpcRequest>,
    ) -> Result<Response<Self::ClientKeygenStream>, Status> {
        let req = request.into_inner();
        tracing::info!("Received keygen request from {}", req.owner_id);
        let (ostream, istream) = mpsc::channel::<Result<MpcProgress, Status>>(16);
        let res = algo_keygen(
            &ostream,
            &self.keystore_dir,
            &req.owner_id,
            &self.lbm_server_url,
            &req.lbm_session_id,
        )
        .await
        .catch_();
        if let Err(e) = res {
            tracing::error!("Keygen failed.\n{}", e);
            return Err(Status::internal(""));
        }

        let resp = Box::pin(ReceiverStream::new(istream));
        Ok(Response::new(resp))
    }

    async fn client_share_root_key(
        &self,
        request: Request<KeySharingRequest>,
    ) -> Result<Response<Self::ClientShareRootKeyStream>, Status> {
        let req = request.into_inner();
        let (ostream, istream) = mpsc::channel::<Result<MpcProgress, Status>>(16);

        let mut phrase = String::new();
        let mut password = String::new();
        if let Some(root_mnem) = &req.root_mnemonic {
            phrase = root_mnem.root_mnemonic.clone();
            if let Some(pwd) = &root_mnem.password {
                password = pwd.clone();
            }
        }

        let res = algo_keygen_root_mnem(
            &ostream,
            &self.keystore_dir,
            &req.sid_oid.owner_id,
            &self.lbm_server_url,
            &req.sid_oid.lbm_session_id,
            &phrase,
            &password,
        )
        .await
        .catch_();
        if let Err(e) = res {
            tracing::error!("ShareRootMnem(KeygenWithMnem) failed.\n{}", e);
            return Err(Status::internal(""));
        }

        let resp = Box::pin(ReceiverStream::new(istream));
        Ok(Response::new(resp))
    }

    async fn client_rescue_key(
        &self,
        _request: Request<KeyRescueRequest>,
    ) -> Result<Response<Self::ClientRescueKeyStream>, Status> {
        unimplemented!("This is the so-called `keygenDumbDumb`")
    }

    async fn client_sign(
        &self,
        request: Request<BasicMpcRequest>,
    ) -> Result<Response<Self::ClientSignStream>, Status> {
        let req = request.into_inner();
        let (ostream, istream) = mpsc::channel::<Result<MpcProgress, Status>>(16);

        let res = algo_sign(
            &ostream,
            &self.keystore_dir,
            &req.owner_id,
            &self.lbm_server_url,
            &req.lbm_session_id,
        )
        .await
        .catch_();
        if let Err(e) = res {
            tracing::error!("Sign failed. {}", e);
            return Err(Status::internal(""));
        }

        let resp = Box::pin(ReceiverStream::new(istream));
        Ok(Response::new(resp))
    }

    async fn client_disagree(
        &self,
        request: Request<BasicMpcRequest>,
    ) -> Result<Response<Void>, Status> {
        let req = request.into_inner();
        let res =
            LubanManagerClient::refuse(&self.lbm_server_url, &req.lbm_session_id, &req.owner_id)
                .await
                .catch_();
        if let Err(e) = res {
            tracing::error!("Failed to reject a session. {}", e);
            return Err(Status::internal(""));
        }

        Ok(Response::new(Void {}))
    }
}
