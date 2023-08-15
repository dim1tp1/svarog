#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicMpcRequest {
    #[prost(string, required, tag = "1")]
    pub lbm_session_id: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RootMnemonic {
    #[prost(string, required, tag = "1")]
    pub root_mnemonic: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeySharingRequest {
    #[prost(message, required, tag = "1")]
    pub sid_oid: BasicMpcRequest,
    #[prost(message, optional, tag = "3")]
    pub root_mnemonic: ::core::option::Option<RootMnemonic>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRescueRequest {
    #[prost(message, required, tag = "1")]
    pub sid_oid: BasicMpcRequest,
    #[prost(string, optional, tag = "2")]
    pub shard_mnemonic: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardMnemonic {
    #[prost(string, required, tag = "1")]
    pub shard_mnemonic: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyMetadata {
    #[prost(string, required, tag = "1")]
    pub root_xpub: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub shard_mnemonic: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MpcProgress {
    #[prost(uint64, required, tag = "1")]
    pub current: u64,
    #[prost(uint64, required, tag = "2")]
    pub total: u64,
    #[prost(string, required, tag = "3")]
    pub message: ::prost::alloc::string::String,
    #[prost(oneof = "mpc_progress::Harvest", tags = "4, 5")]
    pub harvest: ::core::option::Option<mpc_progress::Harvest>,
}
/// Nested message and enum types in `MpcProgress`.
pub mod mpc_progress {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Harvest {
        #[prost(message, tag = "4")]
        KeyMeta(super::KeyMetadata),
        #[prost(message, tag = "5")]
        Signature(super::super::gg18::Signature),
    }
}
/// Generated client implementations.
pub mod luban_client_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LubanClientClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LubanClientClient<tonic::transport::Channel> {
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
    impl<T> LubanClientClient<T>
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
        ) -> LubanClientClient<InterceptedService<T, F>>
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
            LubanClientClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn client_keygen(
            &mut self,
            request: impl tonic::IntoRequest<super::BasicMpcRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MpcProgress>>,
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
                "/lbcs.LubanClient/ClientKeygen",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbcs.LubanClient", "ClientKeygen"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn client_share_root_key(
            &mut self,
            request: impl tonic::IntoRequest<super::KeySharingRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MpcProgress>>,
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
                "/lbcs.LubanClient/ClientShareRootKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbcs.LubanClient", "ClientShareRootKey"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn client_rescue_key(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyRescueRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MpcProgress>>,
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
                "/lbcs.LubanClient/ClientRescueKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbcs.LubanClient", "ClientRescueKey"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn client_sign(
            &mut self,
            request: impl tonic::IntoRequest<super::BasicMpcRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MpcProgress>>,
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
                "/lbcs.LubanClient/ClientSign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbcs.LubanClient", "ClientSign"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn client_disagree(
            &mut self,
            request: impl tonic::IntoRequest<super::BasicMpcRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::lbm::Void>,
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
                "/lbcs.LubanClient/ClientDisagree",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lbcs.LubanClient", "ClientDisagree"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod luban_client_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LubanClientServer.
    #[async_trait]
    pub trait LubanClient: Send + Sync + 'static {
        /// Server streaming response type for the ClientKeygen method.
        type ClientKeygenStream: futures_core::Stream<
                Item = std::result::Result<super::MpcProgress, tonic::Status>,
            >
            + Send
            + 'static;
        async fn client_keygen(
            &self,
            request: tonic::Request<super::BasicMpcRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ClientKeygenStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ClientShareRootKey method.
        type ClientShareRootKeyStream: futures_core::Stream<
                Item = std::result::Result<super::MpcProgress, tonic::Status>,
            >
            + Send
            + 'static;
        async fn client_share_root_key(
            &self,
            request: tonic::Request<super::KeySharingRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ClientShareRootKeyStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ClientRescueKey method.
        type ClientRescueKeyStream: futures_core::Stream<
                Item = std::result::Result<super::MpcProgress, tonic::Status>,
            >
            + Send
            + 'static;
        async fn client_rescue_key(
            &self,
            request: tonic::Request<super::KeyRescueRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ClientRescueKeyStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ClientSign method.
        type ClientSignStream: futures_core::Stream<
                Item = std::result::Result<super::MpcProgress, tonic::Status>,
            >
            + Send
            + 'static;
        async fn client_sign(
            &self,
            request: tonic::Request<super::BasicMpcRequest>,
        ) -> std::result::Result<tonic::Response<Self::ClientSignStream>, tonic::Status>;
        async fn client_disagree(
            &self,
            request: tonic::Request<super::BasicMpcRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::lbm::Void>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LubanClientServer<T: LubanClient> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LubanClient> LubanClientServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LubanClientServer<T>
    where
        T: LubanClient,
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
                "/lbcs.LubanClient/ClientKeygen" => {
                    #[allow(non_camel_case_types)]
                    struct ClientKeygenSvc<T: LubanClient>(pub Arc<T>);
                    impl<
                        T: LubanClient,
                    > tonic::server::ServerStreamingService<super::BasicMpcRequest>
                    for ClientKeygenSvc<T> {
                        type Response = super::MpcProgress;
                        type ResponseStream = T::ClientKeygenStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BasicMpcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).client_keygen(request).await
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
                        let method = ClientKeygenSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbcs.LubanClient/ClientShareRootKey" => {
                    #[allow(non_camel_case_types)]
                    struct ClientShareRootKeySvc<T: LubanClient>(pub Arc<T>);
                    impl<
                        T: LubanClient,
                    > tonic::server::ServerStreamingService<super::KeySharingRequest>
                    for ClientShareRootKeySvc<T> {
                        type Response = super::MpcProgress;
                        type ResponseStream = T::ClientShareRootKeyStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeySharingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).client_share_root_key(request).await
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
                        let method = ClientShareRootKeySvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbcs.LubanClient/ClientRescueKey" => {
                    #[allow(non_camel_case_types)]
                    struct ClientRescueKeySvc<T: LubanClient>(pub Arc<T>);
                    impl<
                        T: LubanClient,
                    > tonic::server::ServerStreamingService<super::KeyRescueRequest>
                    for ClientRescueKeySvc<T> {
                        type Response = super::MpcProgress;
                        type ResponseStream = T::ClientRescueKeyStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeyRescueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).client_rescue_key(request).await
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
                        let method = ClientRescueKeySvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbcs.LubanClient/ClientSign" => {
                    #[allow(non_camel_case_types)]
                    struct ClientSignSvc<T: LubanClient>(pub Arc<T>);
                    impl<
                        T: LubanClient,
                    > tonic::server::ServerStreamingService<super::BasicMpcRequest>
                    for ClientSignSvc<T> {
                        type Response = super::MpcProgress;
                        type ResponseStream = T::ClientSignStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BasicMpcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).client_sign(request).await };
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
                        let method = ClientSignSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lbcs.LubanClient/ClientDisagree" => {
                    #[allow(non_camel_case_types)]
                    struct ClientDisagreeSvc<T: LubanClient>(pub Arc<T>);
                    impl<
                        T: LubanClient,
                    > tonic::server::UnaryService<super::BasicMpcRequest>
                    for ClientDisagreeSvc<T> {
                        type Response = super::super::lbm::Void;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BasicMpcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).client_disagree(request).await
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
                        let method = ClientDisagreeSvc(inner);
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
    impl<T: LubanClient> Clone for LubanClientServer<T> {
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
    impl<T: LubanClient> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LubanClient> tonic::server::NamedService for LubanClientServer<T> {
        const NAME: &'static str = "lbcs.LubanClient";
    }
}
