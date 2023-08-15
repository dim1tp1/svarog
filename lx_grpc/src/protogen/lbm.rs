#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewKeygenSession {
    /// n_keygen := owners.len()
    #[prost(string, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, required, tag = "2")]
    pub min_num_owners: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSignSession {
    #[prost(string, required, tag = "1")]
    pub key_id: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub derv_path: ::prost::alloc::string::String,
    #[prost(bytes = "vec", required, tag = "3")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeygenParameters {
    #[prost(uint32, required, tag = "1")]
    pub threshold: u32,
    #[prost(uint32, required, tag = "2")]
    pub n_actual: u32,
    #[prost(uint32, required, tag = "3")]
    pub n_keygen: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignParameters {
    #[prost(uint32, required, tag = "1")]
    pub threshold: u32,
    #[prost(uint32, required, tag = "2")]
    pub n_actual: u32,
    #[prost(uint32, required, tag = "3")]
    pub n_keygen: u32,
    #[prost(string, required, tag = "4")]
    pub key_id: ::prost::alloc::string::String,
    #[prost(string, required, tag = "5")]
    pub derv_path: ::prost::alloc::string::String,
    #[prost(bytes = "vec", required, tag = "6")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSessionResponse {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(int64, required, tag = "2")]
    pub expire_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttendRequest {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttendResponse {
    #[prost(int64, required, tag = "1")]
    pub expire_at: i64,
    #[prost(uint32, required, tag = "2")]
    pub party_id: u32,
    #[prost(oneof = "attend_response::Params", tags = "3, 4")]
    pub params: ::core::option::Option<attend_response::Params>,
}
/// Nested message and enum types in `AttendResponse`.
pub mod attend_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Params {
        #[prost(message, tag = "3")]
        KeygenParams(super::KeygenParameters),
        #[prost(message, tag = "4")]
        SignParams(super::SignParameters),
    }
}
/// MpcPull
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageToPull {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "2")]
    pub round: u32,
    #[prost(uint32, required, tag = "3")]
    pub party_from: u32,
    #[prost(uint32, required, tag = "4")]
    pub party_to: u32,
}
/// MpcPullFeedback
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePulled {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// MpcPush
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageToPush {
    #[prost(uint32, required, tag = "1")]
    pub party_from: u32,
    #[prost(uint32, required, tag = "2")]
    pub party_to: u32,
    #[prost(uint32, required, tag = "3")]
    pub round: u32,
    #[prost(string, required, tag = "4")]
    pub session_id: ::prost::alloc::string::String,
    /// Use the crate `bincode` to (de)serialize the payload
    #[prost(bytes = "vec", required, tag = "5")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeygenTermination {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "2")]
    pub party_id: u32,
    #[prost(string, required, tag = "3")]
    pub root_xpub: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignTermination {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "2")]
    pub party_id: u32,
    #[prost(message, required, tag = "5")]
    pub sig: super::gg18::Signature,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
    #[prost(bytes = "vec", required, tag = "1")]
    pub raw_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushTxRequest {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
    #[prost(message, required, tag = "3")]
    pub tx: Tx,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressRequest {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullTxRequest {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressPerOwner {
    #[prost(uint32, optional, tag = "1")]
    pub party_id: ::core::option::Option<u32>,
    /// mysql enum('unknown', 'agree', 'disagree')
    #[prost(string, required, tag = "2")]
    pub opinion: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "3")]
    pub round: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeygenProgressPerSession {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    /// map<owner_id, progress>
    #[prost(map = "string, message", tag = "3")]
    pub progress: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ProgressPerOwner,
    >,
    /// keygen finished iff root_xpub is not-null.
    #[prost(string, optional, tag = "4")]
    pub root_xpub: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignProgressPerSession {
    #[prost(string, required, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    /// map<owner_id, progress>
    #[prost(map = "string, message", tag = "3")]
    pub progress: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ProgressPerOwner,
    >,
    /// keygen finished iff root_xpub is not-null.
    #[prost(message, optional, tag = "4")]
    pub signature: ::core::option::Option<super::gg18::Signature>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Void {}
/// Generated client implementations.
pub mod luban_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LubanManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LubanManagerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LubanManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> LubanManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            LubanManagerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn biz_new_keygen_session(
            &mut self,
            request: impl tonic::IntoRequest<super::NewKeygenSession>,
        ) -> std::result::Result<
            tonic::Response<super::NewSessionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/BizNewKeygenSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "BizNewKeygenSession"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn biz_poll_keygen_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::ProgressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeygenProgressPerSession>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/BizPollKeygenProgress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "BizPollKeygenProgress"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn biz_new_sign_session(
            &mut self,
            request: impl tonic::IntoRequest<super::NewSignSession>,
        ) -> std::result::Result<
            tonic::Response<super::NewSessionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/BizNewSignSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "BizNewSignSession"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn biz_poll_sign_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::ProgressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignProgressPerSession>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/BizPollSignProgress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "BizPollSignProgress"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mpc_push_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::PushTxRequest>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/MpcPushTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "MpcPushTx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mpc_pull_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::PullTxRequest>,
        ) -> std::result::Result<tonic::Response<super::Tx>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/MpcPullTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "MpcPullTx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mpc_terminate_keygen(
            &mut self,
            request: impl tonic::IntoRequest<super::KeygenTermination>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/MpcTerminateKeygen",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "MpcTerminateKeygen"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mpc_terminate_sign(
            &mut self,
            request: impl tonic::IntoRequest<super::SignTermination>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/MpcTerminateSign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "MpcTerminateSign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mpc_attend_agree(
            &mut self,
            request: impl tonic::IntoRequest<super::AttendRequest>,
        ) -> std::result::Result<tonic::Response<super::AttendResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/MpcAttendAgree",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "MpcAttendAgree"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mpc_attend_disagree(
            &mut self,
            request: impl tonic::IntoRequest<super::AttendRequest>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/MpcAttendDisagree",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "MpcAttendDisagree"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mpc_pull_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageToPull>,
        ) -> std::result::Result<tonic::Response<super::MessagePulled>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/MpcPullMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "MpcPullMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mpc_push_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageToPush>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/lbm.LubanManager/MpcPushMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbm.LubanManager", "MpcPushMessage"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod luban_manager_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LubanManagerServer.
    #[async_trait]
    pub trait LubanManager: Send + Sync + 'static {
        async fn biz_new_keygen_session(
            &self,
            request: tonic::Request<super::NewKeygenSession>,
        ) -> std::result::Result<
            tonic::Response<super::NewSessionResponse>,
            tonic::Status,
        >;
        async fn biz_poll_keygen_progress(
            &self,
            request: tonic::Request<super::ProgressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeygenProgressPerSession>,
            tonic::Status,
        >;
        async fn biz_new_sign_session(
            &self,
            request: tonic::Request<super::NewSignSession>,
        ) -> std::result::Result<
            tonic::Response<super::NewSessionResponse>,
            tonic::Status,
        >;
        async fn biz_poll_sign_progress(
            &self,
            request: tonic::Request<super::ProgressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignProgressPerSession>,
            tonic::Status,
        >;
        async fn mpc_push_tx(
            &self,
            request: tonic::Request<super::PushTxRequest>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status>;
        async fn mpc_pull_tx(
            &self,
            request: tonic::Request<super::PullTxRequest>,
        ) -> std::result::Result<tonic::Response<super::Tx>, tonic::Status>;
        async fn mpc_terminate_keygen(
            &self,
            request: tonic::Request<super::KeygenTermination>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status>;
        async fn mpc_terminate_sign(
            &self,
            request: tonic::Request<super::SignTermination>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status>;
        async fn mpc_attend_agree(
            &self,
            request: tonic::Request<super::AttendRequest>,
        ) -> std::result::Result<tonic::Response<super::AttendResponse>, tonic::Status>;
        async fn mpc_attend_disagree(
            &self,
            request: tonic::Request<super::AttendRequest>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status>;
        async fn mpc_pull_message(
            &self,
            request: tonic::Request<super::MessageToPull>,
        ) -> std::result::Result<tonic::Response<super::MessagePulled>, tonic::Status>;
        async fn mpc_push_message(
            &self,
            request: tonic::Request<super::MessageToPush>,
        ) -> std::result::Result<tonic::Response<super::Void>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct LubanManagerServer<T: LubanManager> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LubanManager> LubanManagerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LubanManagerServer<T>
    where
        T: LubanManager,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/lbm.LubanManager/BizNewKeygenSession" => {
                    #[allow(non_camel_case_types)]
                    struct BizNewKeygenSessionSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::NewKeygenSession>
                    for BizNewKeygenSessionSvc<T> {
                        type Response = super::NewSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewKeygenSession>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).biz_new_keygen_session(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BizNewKeygenSessionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/BizPollKeygenProgress" => {
                    #[allow(non_camel_case_types)]
                    struct BizPollKeygenProgressSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::ProgressRequest>
                    for BizPollKeygenProgressSvc<T> {
                        type Response = super::KeygenProgressPerSession;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProgressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).biz_poll_keygen_progress(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BizPollKeygenProgressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/BizNewSignSession" => {
                    #[allow(non_camel_case_types)]
                    struct BizNewSignSessionSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::NewSignSession>
                    for BizNewSignSessionSvc<T> {
                        type Response = super::NewSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewSignSession>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).biz_new_sign_session(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BizNewSignSessionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/BizPollSignProgress" => {
                    #[allow(non_camel_case_types)]
                    struct BizPollSignProgressSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::ProgressRequest>
                    for BizPollSignProgressSvc<T> {
                        type Response = super::SignProgressPerSession;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProgressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).biz_poll_sign_progress(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BizPollSignProgressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/MpcPushTx" => {
                    #[allow(non_camel_case_types)]
                    struct MpcPushTxSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::PushTxRequest>
                    for MpcPushTxSvc<T> {
                        type Response = super::Void;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PushTxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).mpc_push_tx(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MpcPushTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/MpcPullTx" => {
                    #[allow(non_camel_case_types)]
                    struct MpcPullTxSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::PullTxRequest>
                    for MpcPullTxSvc<T> {
                        type Response = super::Tx;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PullTxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).mpc_pull_tx(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MpcPullTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/MpcTerminateKeygen" => {
                    #[allow(non_camel_case_types)]
                    struct MpcTerminateKeygenSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::KeygenTermination>
                    for MpcTerminateKeygenSvc<T> {
                        type Response = super::Void;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeygenTermination>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mpc_terminate_keygen(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MpcTerminateKeygenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/MpcTerminateSign" => {
                    #[allow(non_camel_case_types)]
                    struct MpcTerminateSignSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::SignTermination>
                    for MpcTerminateSignSvc<T> {
                        type Response = super::Void;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignTermination>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mpc_terminate_sign(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MpcTerminateSignSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/MpcAttendAgree" => {
                    #[allow(non_camel_case_types)]
                    struct MpcAttendAgreeSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::AttendRequest>
                    for MpcAttendAgreeSvc<T> {
                        type Response = super::AttendResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AttendRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mpc_attend_agree(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MpcAttendAgreeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/MpcAttendDisagree" => {
                    #[allow(non_camel_case_types)]
                    struct MpcAttendDisagreeSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::AttendRequest>
                    for MpcAttendDisagreeSvc<T> {
                        type Response = super::Void;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AttendRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mpc_attend_disagree(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MpcAttendDisagreeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/MpcPullMessage" => {
                    #[allow(non_camel_case_types)]
                    struct MpcPullMessageSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::MessageToPull>
                    for MpcPullMessageSvc<T> {
                        type Response = super::MessagePulled;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessageToPull>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mpc_pull_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MpcPullMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbm.LubanManager/MpcPushMessage" => {
                    #[allow(non_camel_case_types)]
                    struct MpcPushMessageSvc<T: LubanManager>(pub Arc<T>);
                    impl<
                        T: LubanManager,
                    > tonic::server::UnaryService<super::MessageToPush>
                    for MpcPushMessageSvc<T> {
                        type Response = super::Void;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessageToPush>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mpc_push_message(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MpcPushMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: LubanManager> Clone for LubanManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: LubanManager> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LubanManager> tonic::server::NamedService for LubanManagerServer<T> {
        const NAME: &'static str = "lbm.LubanManager";
    }
}
