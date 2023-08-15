use blake2::{digest::consts::U8, Blake2b, Digest};
use clap::{arg, Command};
use lx_grpc::grpc_assert;
use sqlx::{mysql::MySqlPoolOptions, query, MySqlPool, Row};
use std::collections::HashMap;
use xuanmi_base_support::*;

// grpc service
use lx_grpc::lbm::luban_manager_server::{LubanManager, LubanManagerServer};
// grpc types related to initiating new sessions
use lx_grpc::lbm::{NewKeygenSession, NewSessionResponse, NewSignSession};
// grpc types related to polling progress
use lx_grpc::lbm::{
    KeygenProgressPerSession, ProgressPerOwner, ProgressRequest, SignProgressPerSession,
};
// grpc types related to MPC parameters
use lx_grpc::lbm::{KeygenParameters, SignParameters};
// grpc types related to termination
use lx_grpc::lbm::{KeygenTermination, SignTermination};
// grpc types related to TX sharing
use lx_grpc::lbm::{PullTxRequest, PushTxRequest, Tx};
// grpc types related to MPC attendance
use lx_grpc::lbm::{
    attend_response::Params as AttendResponseParams, AttendRequest, AttendResponse,
};
// grpc types related to MPC messenging
pub use lx_grpc::gg18::Signature;
use lx_grpc::lbm::{MessagePulled, MessageToPull, MessageToPush};
// developer utils
// use lx_grpc::prelude::prost::{self, Message}; // provides raw byts encoding/decoding
use lx_grpc::lbm::Void;
use lx_grpc::prelude::tokio;
use lx_grpc::prelude::tonic::{self, transport::Server, Request, Response, Status};

/// Global status of luban messenger.
#[derive(Clone)]
pub struct LubanManagerService {
    pub mysql_pool: MySqlPool,
}

impl LubanManagerService {
    pub async fn new() -> Outcome<Self> {
        let mysql_pool = {
            let mysql_url = format!(
                "mysql://{}:{}@{}:{}/lbm",
                std::env::var("XUANWU_MYSQL_USER").unwrap(),
                std::env::var("XUANWU_MYSQL_PASS").unwrap(),
                std::env::var("XUANWU_MYSQL_HOST").unwrap(),
                std::env::var("XUANWU_MYSQL_PORT").unwrap(),
            );
            MySqlPoolOptions::new()
                .max_connections(128)
                .connect(&mysql_url)
                .await
                .catch_()?
        };

        let created = Self { mysql_pool };
        Ok(created)
    }
}

#[tokio::main]
async fn main() {
    let matches = Command::new("Luban Manager")
        .about("Luban Manager.")
        .arg(
            arg!(-p --port <LOCAL_PORT>)
                .required(false)
                .value_parser(clap::value_parser!(u16))
                .default_value("9000"),
        )
        .arg(
            arg!(--log <LOG_LEVEL>)
                .required(false)
                .default_value("info"),
        )
        .get_matches();
    let p = matches.get_one::<u16>("port").unwrap().to_owned();
    let hostport = format!("127.0.0.1:{}", p);
    let log_level = matches.get_one::<String>("log").unwrap().to_owned();
    init_tracer!("logs", "lbm.log", &log_level);
    tracing::info!("Luban Manager Service will listen on {}", hostport);

    let res = LubanManagerService::new().await;
    if let Err(e) = res {
        tracing::error!("Failed to create LubanManagerService.\n{}", e);
        return;
    }
    println!("Luban Manager is running on {}", &hostport);

    let service = res.unwrap();
    let res = Server::builder()
        .add_service(LubanManagerServer::new(service))
        .serve(hostport.parse().unwrap())
        .await;
    if let Err(e) = res {
        tracing::error!("LBM service is down.\n{}", e);
    }
}

pub fn temp_table_name() -> String {
    type Blake2b32 = Blake2b<U8>;
    let mut hasher = Blake2b32::new();
    hasher.update(uuid::Uuid::new_v4().into_bytes());
    let res = hasher.finalize();

    let x = hex::encode(res);
    format!("temp_{}", &x)
}

#[tonic::async_trait]
impl LubanManager for LubanManagerService {
    async fn biz_new_keygen_session(
        &self,
        request: Request<NewKeygenSession>,
    ) -> Result<Response<NewSessionResponse>, Status> {
        let req = request.into_inner();
        grpc_assert!(req.owners.len() >= req.min_num_owners as usize);
        grpc_assert!(!req.owners.is_empty());

        tracing::info!(
            "BizNewKeygenSession\nmin_num_owners: {}\nowners: {:?}",
            &req.min_num_owners,
            &req.owners
        );

        let res = self.mysql_pool.begin().await;
        if let Err(e) = res {
            tracing::error!("Failed to start transaction.\n{}", e);
            return Err(Status::internal(""));
        }
        let mut trans = res.unwrap();

        let temp_table = temp_table_name();
        let sql = format!(
            "create temporary table {} ( owner_id char(36) ) engine = memory;",
            &temp_table
        );
        let res = query(&sql).execute(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to create temporary table {}.\n{}", &temp_table, e);
            return Err(Status::internal(""));
        }

        let sql = format!("insert into {} values (?)", &temp_table);
        for owner_id in &req.owners {
            let res = query(&sql).bind(owner_id).execute(&mut trans).await;
            if let Err(e) = res {
                tracing::error!("Failed to insert owner_id={}.\n{}", &owner_id, e);
                return Err(Status::internal(""));
            }
        }

        let sql = r#"call new_keygen_session(
            ?, ?, @session_id, @expire_at
        )"#;
        let res = query(sql)
            .bind(req.min_num_owners - 1)
            .bind(&temp_table) // pass table name as string parameter to the stored procedure
            .execute(&mut trans)
            .await;
        if let Err(e) = res {
            tracing::error!(
                "Failed to call mysql procedure `new_keygen_session`.\n{}",
                e
            );
            return Err(Status::internal(""));
        }

        let sql = "select @session_id, @expire_at";
        let res = query(sql).fetch_one(&mut trans).await;
        if let Err(e) = res {
            tracing::error!(
                "Failed to fetch result of mysql procedure `new_keygen_session`.\n{}",
                e
            );
            return Err(Status::internal(""));
        }
        let row = res.unwrap();
        let resp = NewSessionResponse {
            session_id: row.get("@session_id"),
            expire_at: row.get("@expire_at"),
        };

        let sql = format!("drop table {}", &temp_table);
        let res = query(&sql).execute(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to drop temporary table {}.\n{}", &temp_table, e);
            return Err(Status::internal(""));
        }

        let res = trans.commit().await;
        if let Err(e) = res {
            tracing::error!("Failed to commit transaction.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(resp));
    }

    async fn biz_poll_keygen_progress(
        &self,
        request: Request<ProgressRequest>,
    ) -> Result<Response<KeygenProgressPerSession>, Status> {
        let req = request.into_inner();
        let mut resp = KeygenProgressPerSession {
            session_id: req.session_id.clone(),
            progress: HashMap::new(),
            root_xpub: None,
        };
        tracing::info!("BizPollKeygenProgress\nsession_id: {}", &req.session_id);

        // begin transaction
        let res = self.mysql_pool.begin().await;
        if let Err(e) = res {
            tracing::error!("Failed to start transaction.\n{}", e);
            return Err(Status::internal(""));
        }
        let mut trans = res.unwrap();

        let temp_table = temp_table_name();
        let sql = format!(
            r"create temporary table {} (
                owner_id char(36) not null,
                party_id int unsigned,
                opinion char(16) not null,
                round int unsigned not null
            ) engine = memory;",
            &temp_table
        );

        let res = query(&sql).execute(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to create temporary table {}.\n{}", &temp_table, e);
            return Err(Status::internal(""));
        }

        let sql = "call get_keygen_progress(?, ?, @root_xpub)";
        let res = query(sql)
            .bind(req.session_id)
            .bind(&temp_table) // pass table name as string parameter to the stored procedure
            .execute(&mut trans)
            .await;
        if let Err(e) = res {
            tracing::error!(
                "Failed to call mysql procedure `get_keygen_progress`.\n{}",
                e
            );
            return Err(Status::internal(""));
        }

        let sql = "select @root_xpub";
        let res = query(sql).fetch_one(&mut trans).await;
        if let Err(e) = res {
            tracing::error!(
                "Failed to fetch result of mysql procedure `get_keygen_progress`.\n{}",
                e
            );
            return Err(Status::internal(""));
        }
        let row = res.unwrap();
        resp.root_xpub = row.get("@root_xpub");

        let sql = format!(
            "select owner_id, party_id, opinion, round from {}",
            &temp_table
        );
        let res = query(&sql).fetch_all(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to fetch progress from temporary table.\n{}", e);
            return Err(Status::internal(""));
        }
        let rows = res.unwrap();
        for row in &rows {
            resp.progress.insert(
                row.get("owner_id"),
                ProgressPerOwner {
                    party_id: row.get("party_id"),
                    opinion: row.get("opinion"),
                    round: row.get("round"),
                },
            );
        }

        let sql = format!("drop table {}", &temp_table);
        let res = query(&sql).execute(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to drop temporary table {}.\n{}", &temp_table, e);
            return Err(Status::internal(""));
        }

        // commit transaction
        let res = trans.commit().await;
        if let Err(e) = res {
            tracing::error!("Failed to commit transaction.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(resp));
    }

    async fn biz_new_sign_session(
        &self,
        request: Request<NewSignSession>,
    ) -> Result<Response<NewSessionResponse>, Status> {
        let req = request.into_inner();

        // begin transaction
        let res = self.mysql_pool.begin().await;
        if let Err(e) = res {
            tracing::error!("Failed to start transaction.\n{}", e);
            return Err(Status::internal(""));
        }
        let mut trans = res.unwrap();

        let sql = "call new_sign_session(?, ?, ?, @session_id, @expire_at)";
        let res = query(sql)
            .bind(&req.key_id)
            .bind(&req.derv_path)
            .bind(&req.tx_hash)
            .execute(&mut trans)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to call mysql procedure `new_sign_session`.\nkey_id={},derv_path={}\ntx_hash={}.\n{}",
                &req.key_id,
                &req.derv_path,
                hex::encode(&req.tx_hash),
                e
            );
            return Err(Status::internal(""));
        }

        let sql = "select @session_id, @expire_at";
        let res = query(sql).fetch_one(&mut trans).await;
        if let Err(e) = res {
            tracing::error!(
                "Failed to fetch result of mysql procedure `new_sign_session`.\n{}",
                e
            );
            return Err(Status::internal(""));
        }
        let row = res.unwrap();
        let resp = NewSessionResponse {
            session_id: row.get("@session_id"),
            expire_at: row.get("@expire_at"),
        };

        // commit transaction
        let res = trans.commit().await;
        if let Err(e) = res {
            tracing::error!("Failed to commit transaction.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(resp));
    }

    async fn biz_poll_sign_progress(
        &self,
        request: Request<ProgressRequest>,
    ) -> Result<Response<SignProgressPerSession>, Status> {
        let session_id = request.into_inner().session_id;
        let mut resp = SignProgressPerSession {
            session_id: session_id.clone(),
            progress: HashMap::new(),
            signature: None,
        };

        // begin transaction
        let res = self.mysql_pool.begin().await;
        if let Err(e) = res {
            tracing::error!("Failed to start transaction.\n{}", e);
            return Err(Status::internal(""));
        }
        let mut trans = res.unwrap();

        let temp_table1 = temp_table_name();
        let sql = format!(
            r"create temporary table {} (
                owner_id char(36) not null,
                party_id int unsigned,
                opinion char(16) not null,
                round int unsigned not null
            ) engine = memory;",
            &temp_table1
        );
        let res = query(&sql).execute(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to create temporary table {}.\n{}", &temp_table1, e);
            return Err(Status::internal(""));
        }

        let temp_table2 = temp_table_name();
        let sql = format!(
            r"create temporary table {} (
                tx_hash binary(64) not null,
                r binary(64) not null,
                s binary(64) not null,
                v tinyint(1) not null
            ) engine = memory;",
            &temp_table2
        );
        let res = query(&sql).execute(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to create temporary table {}.\n{}", &temp_table2, e);
            return Err(Status::internal(""));
        }

        let sql = "call get_sign_progress(?, ?, ?)";
        let res = query(sql)
            .bind(&session_id)
            .bind(&temp_table1)
            .bind(&temp_table2)
            .execute(&mut trans)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to call mysql procedure `get_sign_progress`.\n{}", e);
            return Err(Status::internal(""));
        }

        let sql = format!("select * from {}", &temp_table1);
        let rows = query(&sql).fetch_all(&mut trans).await.unwrap();
        for row in &rows {
            resp.progress.insert(
                row.get("owner_id"),
                ProgressPerOwner {
                    party_id: row.get("party_id"),
                    opinion: row.get("opinion"),
                    round: row.get("round"),
                },
            );
        }

        let sql = format!("select * from {}", &temp_table2);
        let row = query(&sql).fetch_optional(&mut trans).await.unwrap();
        if let Some(row) = row {
            resp.signature = Some(Signature {
                r: row.get("r"),
                s: row.get("s"),
                v: row.get("v"),
            });
        }

        let sql = format!("drop table {}", &temp_table1);
        let res = query(&sql).execute(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to drop temporary table {}.\n{}", &temp_table1, e);
            return Err(Status::internal(""));
        }

        let sql = format!("drop table {}", &temp_table2);
        let res = query(&sql).execute(&mut trans).await;
        if let Err(e) = res {
            tracing::error!("Failed to drop temporary table {}.\n{}", &temp_table2, e);
            return Err(Status::internal(""));
        }

        // commit transaction
        let res = trans.commit().await;
        if let Err(e) = res {
            tracing::error!("Failed to commit transaction.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(resp));
    }

    async fn mpc_push_tx(
        &self,
        _request: Request<PushTxRequest>,
    ) -> Result<Response<Void>, Status> {
        let _req = _request.into_inner();
        todo!();
    }

    async fn mpc_pull_tx(&self, _request: Request<PullTxRequest>) -> Result<Response<Tx>, Status> {
        let _req = _request.into_inner();
        todo!();
    }

    async fn mpc_terminate_keygen(
        &self,
        request: Request<KeygenTermination>,
    ) -> Result<Response<Void>, Status> {
        let req = request.into_inner();

        // begin transaction
        let res = self.mysql_pool.begin().await;
        if let Err(e) = res {
            tracing::error!("Failed to start transaction.\n{}", e);
            return Err(Status::internal(""));
        }
        let mut trans = res.unwrap();

        let sql = "call terminate_keygen(?, ?, ?)";
        let res = query(sql)
            .bind(req.session_id)
            .bind(req.party_id)
            .bind(req.root_xpub)
            .execute(&mut trans)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to call mysql procedure `terminate_keygen`.\n{}", e);
            return Err(Status::internal(""));
        }

        // commit transaction
        let res = trans.commit().await;
        if let Err(e) = res {
            tracing::error!("Failed to commit transaction.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(Void {}));
    }

    async fn mpc_terminate_sign(
        &self,
        request: Request<SignTermination>,
    ) -> Result<Response<Void>, Status> {
        let req = request.into_inner();

        // begin transaction
        let res = self.mysql_pool.begin().await;
        if let Err(e) = res {
            tracing::error!("Failed to start transaction.\n{}", e);
            return Err(Status::internal(""));
        }
        let mut trans = res.unwrap();

        let sql = "call terminate_sign(?, ?, ?, ?, ?)";
        let res = query(sql)
            .bind(req.session_id)
            .bind(req.party_id)
            .bind(req.sig.r)
            .bind(req.sig.s)
            .bind(req.sig.v)
            .execute(&mut trans)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to call mysql procedure `terminate_sign`.\n{}", e);
            return Err(Status::internal(""));
        }

        // commit transaction
        let res = trans.commit().await;
        if let Err(e) = res {
            tracing::error!("Failed to commit transaction.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(Void {}));
    }

    async fn mpc_attend_agree(
        &self,
        request: Request<AttendRequest>,
    ) -> Result<Response<AttendResponse>, Status> {
        let req = request.into_inner();

        let mut resp = AttendResponse {
            expire_at: 0,
            party_id: 0,
            params: None,
        };

        // begin transaction
        let res = self.mysql_pool.begin().await;
        if let Err(e) = res {
            tracing::error!("Failed to start transaction.\n{}", e);
            return Err(Status::internal(""));
        }
        let mut trans = res.unwrap();

        let sql = "call attend_agree(?, ?, @session_type, @party_id, @expire_at)";
        let res = query(sql)
            .bind(&req.session_id)
            .bind(&req.owner_id)
            .execute(&mut trans)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to call mysql procedure `attend_agree`.\n{}", e);
            return Err(Status::internal(""));
        }

        let sql = "select @session_type, @party_id, @expire_at";
        let row = query(sql).fetch_one(&mut trans).await.unwrap();
        let session_type: String = row.get("@session_type");
        resp.party_id = row.get("@party_id");
        resp.expire_at = row.get("@expire_at");

        let sql = "select * from mpc_session_params where session_id = ?";
        let row = query(sql)
            .bind(&req.session_id)
            .fetch_one(&mut trans)
            .await
            .unwrap();
        match session_type.as_str() {
            "keygen" => {
                let params = KeygenParameters {
                    threshold: row.get("threshold"),
                    n_actual: row.get("n_keygen"),
                    n_keygen: row.get("n_keygen"),
                };
                resp.params = Some(AttendResponseParams::KeygenParams(params));
            }
            "sign" => {
                let mut params = SignParameters {
                    threshold: row.get("threshold"),
                    n_actual: row.get("threshold"),
                    n_keygen: row.get("n_keygen"),
                    key_id: row.get("key_id"),
                    derv_path: row.get("derv_path"),
                    tx_hash: row.get("tx_hash"),
                };
                params.n_actual += 1;
                resp.params = Some(AttendResponseParams::SignParams(params));
            }
            other => {
                tracing::error!("invalid session type: {}", other);
                return Err(Status::internal(""));
            }
        }

        // commit transaction
        let res = trans.commit().await;
        if let Err(e) = res {
            tracing::error!("Failed to commit transaction.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(resp));
    }

    async fn mpc_attend_disagree(
        &self,
        request: Request<AttendRequest>,
    ) -> Result<Response<Void>, Status> {
        let req = request.into_inner();

        // Only one sql statement, no need to start a transaction.
        let sql = "call attend_disagree(?, ?)";
        let res = query(sql)
            .bind(req.session_id)
            .bind(req.owner_id)
            .execute(&self.mysql_pool)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to call mysql procedure `attend_disagree`.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(Void {}));
    }

    async fn mpc_pull_message(
        &self,
        request: Request<MessageToPull>,
    ) -> Result<Response<MessagePulled>, Status> {
        let req = request.into_inner();
        let mut resp = MessagePulled { message: None };

        // begin transaction
        let res = self.mysql_pool.begin().await;
        if let Err(e) = res {
            tracing::error!("Failed to start transaction.\n{}", e);
            return Err(Status::internal(""));
        }
        let mut trans = res.unwrap();

        let sql = "call pull_mpc_message(?, ?, ?, ?, @message)";
        let res = query(sql)
            .bind(req.session_id)
            .bind(req.round)
            .bind(req.party_from)
            .bind(req.party_to)
            .execute(&mut trans)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to call mysql procedure `pull_mpc_message`.\n{}", e);
            return Err(Status::internal(""));
        }

        let sql = "select @message";
        let row = query(sql).fetch_one(&mut trans).await.unwrap();
        resp.message = row.get("@message");

        // commit transaction
        let res = trans.commit().await;
        if let Err(e) = res {
            tracing::error!("Failed to commit transaction.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(resp));
    }

    async fn mpc_push_message(
        &self,
        request: Request<MessageToPush>,
    ) -> Result<Response<Void>, Status> {
        let req = request.into_inner();

        let sql = "call push_mpc_message(?, ?, ?, ?, ?)";
        let res = query(sql)
            .bind(req.session_id)
            .bind(req.round)
            .bind(req.party_from)
            .bind(req.party_to)
            .bind(&req.message)
            .execute(&self.mysql_pool)
            .await;
        if let Err(e) = res {
            tracing::error!("Failed to call mysql procedure `push_mpc_message`.\n{}", e);
            return Err(Status::internal(""));
        }

        return Ok(Response::new(Void {}));
    }
}
