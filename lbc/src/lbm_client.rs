use chrono::Utc;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use xuanmi_base_support::*;

// grpc client side
use lx_grpc::lbm::luban_manager_client::LubanManagerClient as GrpcClient;
// grpc types related to termination
pub use lx_grpc::lbm::{KeygenTermination, SignTermination};
// grpc types related to TX sharing
pub use lx_grpc::lbm::{PullTxRequest, PushTxRequest, Tx};
// grpc types related to MPC attendance
use lx_grpc::lbm::{attend_response::Params as AttendResponseParams, AttendRequest};
// grpc types related to MPC messenging
pub use lx_grpc::gg18::Signature;
pub use lx_grpc::lbm::{MessagePulled, MessageToPull, MessageToPush};
// developer utils
pub use lx_grpc::prelude::tokio;
pub use lx_grpc::prelude::tokio::sync::mpsc;
pub use lx_grpc::prelude::tonic::{transport::Channel, Request, Response, Status};

pub async fn sleep_async(msec: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(msec)).await;
}

pub struct LubanManagerClient {
    client: GrpcClient<Channel>,
    session_id: String,

    #[allow(dead_code)]
    owner_id: String,

    party_id: usize,
    expire_at: i64,
    params: AttendResponseParams,
}

pub enum TerminationType {
    Keygen(String),
    Sign(Signature),
}

impl LubanManagerClient {
    pub async fn attend(hostport: &str, session_id: &str, owner_id: &str) -> Outcome<Self> {
        let mut client = GrpcClient::connect(hostport.to_owned()).await.catch_()?;

        let req = Request::new(AttendRequest {
            session_id: session_id.to_string(),
            owner_id: owner_id.to_string(),
        });
        let resp = client.mpc_attend_agree(req).await.catch_()?.into_inner();

        Ok(Self {
            client,
            session_id: session_id.to_string(),
            owner_id: owner_id.to_string(),
            party_id: resp.party_id as usize,
            expire_at: resp.expire_at,
            params: resp.params.unwrap(),
        })
    }

    pub async fn refuse(hostport: &str, session_id: &str, owner_id: &str) -> Outcome<()> {
        let mut client = GrpcClient::connect(hostport.to_owned()).await.catch_()?;

        let req = Request::new(AttendRequest {
            session_id: session_id.to_string(),
            owner_id: owner_id.to_string(),
        });
        let _ = client.mpc_attend_agree(req).await.catch_()?;
        Ok(())
    }

    pub async fn send_broadcast<T>(&mut self, round: usize, message: &T) -> Outcome<()>
    where
        T: Serialize + DeserializeOwned,
    {
        if self.is_expired() {
            let ctx = format!("UUID={} expired during {}", &self.session_id, fnn!());
            throw!("BusinessLogicException", &ctx);
        }
        let message = message.compress().catch_()?;
        let req = Request::new(MessageToPush {
            session_id: self.session_id.clone(),
            party_from: self.party_id as u32,
            party_to: 0,
            round: round as u32,
            message: message.clone(),
        });
        let _ = self.client.mpc_push_message(req).await.catch(
            "",
            &format!(
                "uuid={}, party_id={}, round={}",
                &self.session_id, &self.party_id, round
            ),
        )?;
        Ok(())
    }

    pub async fn send_p2p<T>(&mut self, other_id: usize, round: usize, message: &T) -> Outcome<()>
    where
        T: Serialize + DeserializeOwned,
    {
        if self.is_expired() {
            let ctx = format!("UUID={} expired during {}", &self.session_id, fnn!());
            throw!("BusinessLogicException", &ctx);
        }
        let message = message.compress().catch_()?;
        let req = Request::new(MessageToPush {
            session_id: self.session_id.clone(),
            party_from: self.party_id as u32,
            party_to: other_id as u32,
            round: round as u32,
            message: message.clone(),
        });
        let _ = self.client.mpc_push_message(req).await.catch(
            "",
            &format!(
                "uuid={}, party_id={}, round={}",
                &self.session_id, &self.party_id, round
            ),
        )?;
        Ok(())
    }

    pub async fn send_terminate(&mut self, termination: &TerminationType) -> Outcome<()> {
        if self.is_expired() {
            let ctx = format!("UUID={} expired during {}", &self.session_id, fnn!());
            throw!("BusinessLogicException", &ctx);
        }
        let resp = match termination {
            TerminationType::Keygen(root_xpub) => {
                let req = Request::new(KeygenTermination {
                    session_id: self.session_id.clone(),
                    party_id: self.party_id as u32,
                    root_xpub: root_xpub.clone(),
                });
                self.client.mpc_terminate_keygen(req).await
            }
            TerminationType::Sign(sig) => {
                let req = Request::new(SignTermination {
                    session_id: self.session_id.clone(),
                    party_id: self.party_id as u32,
                    sig: sig.clone(),
                });
                self.client.mpc_terminate_sign(req).await
            }
        };
        let _ = resp.catch(
            "",
            &format!(
                "uuid={}, party_id={}, round={}",
                &self.session_id, &self.party_id, "terminate"
            ),
        )?;
        Ok(())
    }

    pub async fn pull_before_expire<T>(&mut self, idx: &MessageToPull) -> Outcome<Option<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let req = Request::new(idx.clone());
        let bytes = self
            .client
            .mpc_pull_message(req)
            .await
            .catch(
                "",
                &format!(
                    "uuid={}, party_id={}, round={}",
                    &self.session_id, &self.party_id, idx.round
                ),
            )?
            .into_inner()
            .message;
        if bytes.is_none() {
            Ok(None)
        } else {
            let bytes = bytes.unwrap();
            let obj = bytes.decompress().catch_()?;
            Ok(Some(obj))
        }
    }

    pub async fn recv_broadcasts<T>(&mut self, round: usize) -> Outcome<HashMap<usize, T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let n_actual = self.n_actual();
        let party_id = self.party_id;
        let mut ret: HashMap<usize, T> = HashMap::with_capacity(n_actual);
        '_outer: for i in 1..=n_actual {
            if i == party_id {
                continue;
            }
            let index = MessageToPull {
                session_id: self.session_id.clone(),
                party_from: i as u32,
                party_to: 0,
                round: round as u32,
            };
            'inner: loop {
                match self.pull_before_expire(&index).await {
                    Ok(Some(obj)) => {
                        let _ = ret.insert(i, obj);
                        break 'inner;
                    }
                    Ok(None) => {
                        sleep_async(200).await;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(ret)
    }

    pub async fn recv_p2p<T>(&mut self, other_id: usize, round: usize) -> Outcome<T>
    where
        T: Serialize + DeserializeOwned,
    {
        let index = MessageToPull {
            session_id: self.session_id.clone(),
            party_from: other_id as u32,
            party_to: self.party_id as u32,
            round: round as u32,
        };

        #[allow(clippy::needless_return)]
        loop {
            match self.pull_before_expire(&index).await {
                Ok(Some(obj)) => {
                    return Ok(obj);
                }
                Ok(None) => {
                    sleep_async(200).await;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    pub async fn recv_all_p2p<T>(&mut self, round: usize) -> Outcome<HashMap<usize, T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut ret: HashMap<usize, T> = HashMap::with_capacity(self.n_actual());
        for other_id in 1..=self.n_actual() {
            if other_id == self.party_id {
                continue;
            }
            match self.recv_p2p(other_id, round).await {
                Ok(obj) => {
                    let _ = ret.insert(other_id, obj);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(ret)
    }

    pub fn session_id(&self) -> &str {
        return self.session_id.as_str();
    }

    /// 获取由 MPC Messenger 分配的 party_id.
    pub fn party_id(&self) -> usize {
        self.party_id
    }

    pub fn n_actual(&self) -> usize {
        match &self.params {
            AttendResponseParams::KeygenParams(var) => var.n_actual as usize,
            AttendResponseParams::SignParams(var) => var.n_actual as usize,
        }
    }

    pub fn n_keygen(&self) -> usize {
        match &self.params {
            AttendResponseParams::KeygenParams(var) => var.n_keygen as usize,
            AttendResponseParams::SignParams(var) => var.n_keygen as usize,
        }
    }

    pub fn threshold(&self) -> usize {
        match &self.params {
            AttendResponseParams::KeygenParams(var) => var.threshold as usize,
            AttendResponseParams::SignParams(var) => var.threshold as usize,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expire_at
    }

    pub fn tx_hash(&self) -> &[u8] {
        match &self.params {
            AttendResponseParams::KeygenParams(_) => panic!("KeygenParams has no tx_hash"),
            AttendResponseParams::SignParams(var) => var.tx_hash.as_slice(),
        }
    }

    pub fn derv_path(&self) -> &str {
        match &self.params {
            AttendResponseParams::KeygenParams(_) => panic!("KeygenParams has no derv_path"),
            AttendResponseParams::SignParams(var) => var.derv_path.as_str(),
        }
    }

    pub fn key_id(&self) -> &str {
        match &self.params {
            AttendResponseParams::KeygenParams(_) => panic!("KeygenParams has no key_id"),
            AttendResponseParams::SignParams(var) => var.key_id.as_str(),
        }
    }
}

trait Compress {
    fn compress(&self) -> Outcome<Vec<u8>>;
}

trait Decompress<T> {
    fn decompress(&self) -> Outcome<T>;
}

use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;

impl<T> Compress for T
where
    T: Serialize + DeserializeOwned,
{
    fn compress(&self) -> Outcome<Vec<u8>> {
        let json = serde_json::to_string(&self).catch_()?;
        let bytes = compress_to_vec(json.as_bytes(), 7);
        Ok(bytes)
    }
}

impl<S, D> Decompress<D> for S
where
    S: AsRef<[u8]>,
    D: Serialize + DeserializeOwned,
{
    fn decompress(&self) -> Outcome<D> {
        let bytes = decompress_to_vec(self.as_ref()).catch_()?;
        let json = String::from_utf8(bytes).catch_()?;
        let obj = serde_json::from_str(&json).catch_()?;
        Ok(obj)
    }
}
