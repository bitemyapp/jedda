/// StdioData is a single chunk of stdout or stderr data that is streamed
/// from GRPCStdio.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StdioData {
    #[prost(enumeration = "stdio_data::Channel", tag = "1")]
    pub channel: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `StdioData`.
pub mod stdio_data {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Channel {
        Invalid = 0,
        Stdout = 1,
        Stderr = 2,
    }
}
#[doc = r" Generated server implementations."]
pub mod grpc_stdio_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GrpcStdioServer."]
    #[async_trait]
    pub trait GrpcStdio: Send + Sync + 'static {
        #[doc = "Server streaming response type for the StreamStdio method."]
        type StreamStdioStream: futures_core::Stream<Item = Result<super::StdioData, tonic::Status>>
            + Send
            + 'static;
        #[doc = " StreamStdio returns a stream that contains all the stdout/stderr."]
        #[doc = " This RPC endpoint must only be called ONCE. Once stdio data is consumed"]
        #[doc = " it is not sent again."]
        #[doc = ""]
        #[doc = " Callers should connect early to prevent blocking on the plugin process."]
        async fn stream_stdio(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<Self::StreamStdioStream>, tonic::Status>;
    }
    #[doc = " GRPCStdio is a service that is automatically run by the plugin process"]
    #[doc = " to stream any stdout/err data so that it can be mirrored on the plugin"]
    #[doc = " host side."]
    #[derive(Debug)]
    pub struct GrpcStdioServer<T: GrpcStdio> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GrpcStdio> GrpcStdioServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GrpcStdioServer<T>
    where
        T: GrpcStdio,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/plugin.GRPCStdio/StreamStdio" => {
                    #[allow(non_camel_case_types)]
                    struct StreamStdioSvc<T: GrpcStdio>(pub Arc<T>);
                    impl<T: GrpcStdio> tonic::server::ServerStreamingService<()> for StreamStdioSvc<T> {
                        type Response = super::StdioData;
                        type ResponseStream = T::StreamStdioStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_stdio(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamStdioSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: GrpcStdio> Clone for GrpcStdioServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GrpcStdio> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GrpcStdio> tonic::transport::NamedService for GrpcStdioServer<T> {
        const NAME: &'static str = "plugin.GRPCStdio";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnInfo {
    #[prost(uint32, tag = "1")]
    pub service_id: u32,
    #[prost(string, tag = "2")]
    pub network: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
}
#[doc = r" Generated server implementations."]
pub mod grpc_broker_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GrpcBrokerServer."]
    #[async_trait]
    pub trait GrpcBroker: Send + Sync + 'static {
        #[doc = "Server streaming response type for the StartStream method."]
        type StartStreamStream: futures_core::Stream<Item = Result<super::ConnInfo, tonic::Status>>
            + Send
            + 'static;
        async fn start_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::ConnInfo>>,
        ) -> Result<tonic::Response<Self::StartStreamStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GrpcBrokerServer<T: GrpcBroker> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GrpcBroker> GrpcBrokerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GrpcBrokerServer<T>
    where
        T: GrpcBroker,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/plugin.GRPCBroker/StartStream" => {
                    #[allow(non_camel_case_types)]
                    struct StartStreamSvc<T: GrpcBroker>(pub Arc<T>);
                    impl<T: GrpcBroker> tonic::server::StreamingService<super::ConnInfo> for StartStreamSvc<T> {
                        type Response = super::ConnInfo;
                        type ResponseStream = T::StartStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ConnInfo>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: GrpcBroker> Clone for GrpcBrokerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GrpcBroker> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GrpcBroker> tonic::transport::NamedService for GrpcBrokerServer<T> {
        const NAME: &'static str = "plugin.GRPCBroker";
    }
}
