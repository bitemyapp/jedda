/// DynamicValue is an opaque encoding of terraform data, with the field name
/// indicating the encoding scheme used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicValue {
    #[prost(bytes = "vec", tag = "1")]
    pub msgpack: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub json: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Diagnostic {
    #[prost(enumeration = "diagnostic::Severity", tag = "1")]
    pub severity: i32,
    #[prost(string, tag = "2")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub detail: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub attribute: ::core::option::Option<AttributePath>,
}
/// Nested message and enum types in `Diagnostic`.
pub mod diagnostic {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        Invalid = 0,
        Error = 1,
        Warning = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributePath {
    #[prost(message, repeated, tag = "1")]
    pub steps: ::prost::alloc::vec::Vec<attribute_path::Step>,
}
/// Nested message and enum types in `AttributePath`.
pub mod attribute_path {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Step {
        #[prost(oneof = "step::Selector", tags = "1, 2, 3")]
        pub selector: ::core::option::Option<step::Selector>,
    }
    /// Nested message and enum types in `Step`.
    pub mod step {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Selector {
            /// Set "attribute_name" to represent looking up an attribute
            /// in the current object value.
            #[prost(string, tag = "1")]
            AttributeName(::prost::alloc::string::String),
            /// Set "element_key_*" to represent looking up an element in
            /// an indexable collection type.
            #[prost(string, tag = "2")]
            ElementKeyString(::prost::alloc::string::String),
            #[prost(int64, tag = "3")]
            ElementKeyInt(i64),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopProvider {}
/// Nested message and enum types in `StopProvider`.
pub mod stop_provider {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub error: ::prost::alloc::string::String,
    }
}
/// RawState holds the stored state for a resource to be upgraded by the
/// provider. It can be in one of two formats, the current json encoded format
/// in bytes, or the legacy flatmap format as a map of strings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawState {
    #[prost(bytes = "vec", tag = "1")]
    pub json: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, string", tag = "2")]
    pub flatmap:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Schema is the configuration schema for a Resource or Provider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// The version of the schema.
    /// Schemas are versioned, so that providers can upgrade a saved resource
    /// state when the schema is changed.
    #[prost(int64, tag = "1")]
    pub version: i64,
    /// Block is the top level configuration block for this schema.
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<schema::Block>,
}
/// Nested message and enum types in `Schema`.
pub mod schema {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Block {
        #[prost(int64, tag = "1")]
        pub version: i64,
        #[prost(message, repeated, tag = "2")]
        pub attributes: ::prost::alloc::vec::Vec<Attribute>,
        #[prost(message, repeated, tag = "3")]
        pub block_types: ::prost::alloc::vec::Vec<NestedBlock>,
        #[prost(string, tag = "4")]
        pub description: ::prost::alloc::string::String,
        #[prost(enumeration = "super::StringKind", tag = "5")]
        pub description_kind: i32,
        #[prost(bool, tag = "6")]
        pub deprecated: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attribute {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "2")]
        pub r#type: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, optional, tag = "10")]
        pub nested_type: ::core::option::Option<Object>,
        #[prost(string, tag = "3")]
        pub description: ::prost::alloc::string::String,
        #[prost(bool, tag = "4")]
        pub required: bool,
        #[prost(bool, tag = "5")]
        pub optional: bool,
        #[prost(bool, tag = "6")]
        pub computed: bool,
        #[prost(bool, tag = "7")]
        pub sensitive: bool,
        #[prost(enumeration = "super::StringKind", tag = "8")]
        pub description_kind: i32,
        #[prost(bool, tag = "9")]
        pub deprecated: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NestedBlock {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub block: ::core::option::Option<Block>,
        #[prost(enumeration = "nested_block::NestingMode", tag = "3")]
        pub nesting: i32,
        #[prost(int64, tag = "4")]
        pub min_items: i64,
        #[prost(int64, tag = "5")]
        pub max_items: i64,
    }
    /// Nested message and enum types in `NestedBlock`.
    pub mod nested_block {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum NestingMode {
            Invalid = 0,
            Single = 1,
            List = 2,
            Set = 3,
            Map = 4,
            Group = 5,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Object {
        #[prost(message, repeated, tag = "1")]
        pub attributes: ::prost::alloc::vec::Vec<Attribute>,
        #[prost(enumeration = "object::NestingMode", tag = "3")]
        pub nesting: i32,
        /// MinItems and MaxItems were never used in the protocol, and have no
        /// effect on validation.
        #[deprecated]
        #[prost(int64, tag = "4")]
        pub min_items: i64,
        #[deprecated]
        #[prost(int64, tag = "5")]
        pub max_items: i64,
    }
    /// Nested message and enum types in `Object`.
    pub mod object {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum NestingMode {
            Invalid = 0,
            Single = 1,
            List = 2,
            Set = 3,
            Map = 4,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProviderSchema {}
/// Nested message and enum types in `GetProviderSchema`.
pub mod get_provider_schema {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub provider: ::core::option::Option<super::Schema>,
        #[prost(map = "string, message", tag = "2")]
        pub resource_schemas:
            ::std::collections::HashMap<::prost::alloc::string::String, super::Schema>,
        #[prost(map = "string, message", tag = "3")]
        pub data_source_schemas:
            ::std::collections::HashMap<::prost::alloc::string::String, super::Schema>,
        #[prost(message, repeated, tag = "4")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
        #[prost(message, optional, tag = "5")]
        pub provider_meta: ::core::option::Option<super::Schema>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateProviderConfig {}
/// Nested message and enum types in `ValidateProviderConfig`.
pub mod validate_provider_config {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub config: ::core::option::Option<super::DynamicValue>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "2")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeResourceState {}
/// Nested message and enum types in `UpgradeResourceState`.
pub mod upgrade_resource_state {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        /// version is the schema_version number recorded in the state file
        #[prost(int64, tag = "2")]
        pub version: i64,
        /// raw_state is the raw states as stored for the resource.  Core does
        /// not have access to the schema of prior_version, so it's the
        /// provider's responsibility to interpret this value using the
        /// appropriate older schema. The raw_state will be the json encoded
        /// state, or a legacy flat-mapped format.
        #[prost(message, optional, tag = "3")]
        pub raw_state: ::core::option::Option<super::RawState>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// new_state is a msgpack-encoded data structure that, when interpreted with
        /// the _current_ schema for this resource type, is functionally equivalent to
        /// that which was given in prior_state_raw.
        #[prost(message, optional, tag = "1")]
        pub upgraded_state: ::core::option::Option<super::DynamicValue>,
        /// diagnostics describes any errors encountered during migration that could not
        /// be safely resolved, and warnings about any possibly-risky assumptions made
        /// in the upgrade process.
        #[prost(message, repeated, tag = "2")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateResourceConfig {}
/// Nested message and enum types in `ValidateResourceConfig`.
pub mod validate_resource_config {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub config: ::core::option::Option<super::DynamicValue>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateDataResourceConfig {}
/// Nested message and enum types in `ValidateDataResourceConfig`.
pub mod validate_data_resource_config {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub config: ::core::option::Option<super::DynamicValue>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureProvider {}
/// Nested message and enum types in `ConfigureProvider`.
pub mod configure_provider {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub terraform_version: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub config: ::core::option::Option<super::DynamicValue>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResource {}
/// Nested message and enum types in `ReadResource`.
pub mod read_resource {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub current_state: ::core::option::Option<super::DynamicValue>,
        #[prost(bytes = "vec", tag = "3")]
        pub private: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, optional, tag = "4")]
        pub provider_meta: ::core::option::Option<super::DynamicValue>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub new_state: ::core::option::Option<super::DynamicValue>,
        #[prost(message, repeated, tag = "2")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
        #[prost(bytes = "vec", tag = "3")]
        pub private: ::prost::alloc::vec::Vec<u8>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanResourceChange {}
/// Nested message and enum types in `PlanResourceChange`.
pub mod plan_resource_change {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub prior_state: ::core::option::Option<super::DynamicValue>,
        #[prost(message, optional, tag = "3")]
        pub proposed_new_state: ::core::option::Option<super::DynamicValue>,
        #[prost(message, optional, tag = "4")]
        pub config: ::core::option::Option<super::DynamicValue>,
        #[prost(bytes = "vec", tag = "5")]
        pub prior_private: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, optional, tag = "6")]
        pub provider_meta: ::core::option::Option<super::DynamicValue>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub planned_state: ::core::option::Option<super::DynamicValue>,
        #[prost(message, repeated, tag = "2")]
        pub requires_replace: ::prost::alloc::vec::Vec<super::AttributePath>,
        #[prost(bytes = "vec", tag = "3")]
        pub planned_private: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag = "4")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyResourceChange {}
/// Nested message and enum types in `ApplyResourceChange`.
pub mod apply_resource_change {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub prior_state: ::core::option::Option<super::DynamicValue>,
        #[prost(message, optional, tag = "3")]
        pub planned_state: ::core::option::Option<super::DynamicValue>,
        #[prost(message, optional, tag = "4")]
        pub config: ::core::option::Option<super::DynamicValue>,
        #[prost(bytes = "vec", tag = "5")]
        pub planned_private: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, optional, tag = "6")]
        pub provider_meta: ::core::option::Option<super::DynamicValue>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub new_state: ::core::option::Option<super::DynamicValue>,
        #[prost(bytes = "vec", tag = "2")]
        pub private: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag = "3")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportResourceState {}
/// Nested message and enum types in `ImportResourceState`.
pub mod import_resource_state {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImportedResource {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub state: ::core::option::Option<super::DynamicValue>,
        #[prost(bytes = "vec", tag = "3")]
        pub private: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub imported_resources: ::prost::alloc::vec::Vec<ImportedResource>,
        #[prost(message, repeated, tag = "2")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadDataSource {}
/// Nested message and enum types in `ReadDataSource`.
pub mod read_data_source {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub type_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub config: ::core::option::Option<super::DynamicValue>,
        #[prost(message, optional, tag = "3")]
        pub provider_meta: ::core::option::Option<super::DynamicValue>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub state: ::core::option::Option<super::DynamicValue>,
        #[prost(message, repeated, tag = "2")]
        pub diagnostics: ::prost::alloc::vec::Vec<super::Diagnostic>,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StringKind {
    Plain = 0,
    Markdown = 1,
}
#[doc = r" Generated server implementations."]
pub mod provider_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProviderServer."]
    #[async_trait]
    pub trait Provider: Send + Sync + 'static {
        #[doc = "////// Information about what a provider supports/expects"]
        async fn get_provider_schema(
            &self,
            request: tonic::Request<super::get_provider_schema::Request>,
        ) -> Result<tonic::Response<super::get_provider_schema::Response>, tonic::Status>;
        async fn validate_provider_config(
            &self,
            request: tonic::Request<super::validate_provider_config::Request>,
        ) -> Result<tonic::Response<super::validate_provider_config::Response>, tonic::Status>;
        async fn validate_resource_config(
            &self,
            request: tonic::Request<super::validate_resource_config::Request>,
        ) -> Result<tonic::Response<super::validate_resource_config::Response>, tonic::Status>;
        async fn validate_data_resource_config(
            &self,
            request: tonic::Request<super::validate_data_resource_config::Request>,
        ) -> Result<tonic::Response<super::validate_data_resource_config::Response>, tonic::Status>;
        async fn upgrade_resource_state(
            &self,
            request: tonic::Request<super::upgrade_resource_state::Request>,
        ) -> Result<tonic::Response<super::upgrade_resource_state::Response>, tonic::Status>;
        #[doc = "////// One-time initialization, called before other functions below"]
        async fn configure_provider(
            &self,
            request: tonic::Request<super::configure_provider::Request>,
        ) -> Result<tonic::Response<super::configure_provider::Response>, tonic::Status>;
        #[doc = "////// Managed Resource Lifecycle"]
        async fn read_resource(
            &self,
            request: tonic::Request<super::read_resource::Request>,
        ) -> Result<tonic::Response<super::read_resource::Response>, tonic::Status>;
        async fn plan_resource_change(
            &self,
            request: tonic::Request<super::plan_resource_change::Request>,
        ) -> Result<tonic::Response<super::plan_resource_change::Response>, tonic::Status>;
        async fn apply_resource_change(
            &self,
            request: tonic::Request<super::apply_resource_change::Request>,
        ) -> Result<tonic::Response<super::apply_resource_change::Response>, tonic::Status>;
        async fn import_resource_state(
            &self,
            request: tonic::Request<super::import_resource_state::Request>,
        ) -> Result<tonic::Response<super::import_resource_state::Response>, tonic::Status>;
        async fn read_data_source(
            &self,
            request: tonic::Request<super::read_data_source::Request>,
        ) -> Result<tonic::Response<super::read_data_source::Response>, tonic::Status>;
        #[doc = "////// Graceful Shutdown"]
        async fn stop_provider(
            &self,
            request: tonic::Request<super::stop_provider::Request>,
        ) -> Result<tonic::Response<super::stop_provider::Response>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ProviderServer<T: Provider> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Provider> ProviderServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProviderServer<T>
    where
        T: Provider,
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
                "/tfplugin6.Provider/GetProviderSchema" => {
                    #[allow(non_camel_case_types)]
                    struct GetProviderSchemaSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::get_provider_schema::Request>
                        for GetProviderSchemaSvc<T>
                    {
                        type Response = super::get_provider_schema::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::get_provider_schema::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_provider_schema(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProviderSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/ValidateProviderConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateProviderConfigSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::validate_provider_config::Request>
                        for ValidateProviderConfigSvc<T>
                    {
                        type Response = super::validate_provider_config::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::validate_provider_config::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).validate_provider_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidateProviderConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/ValidateResourceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateResourceConfigSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::validate_resource_config::Request>
                        for ValidateResourceConfigSvc<T>
                    {
                        type Response = super::validate_resource_config::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::validate_resource_config::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).validate_resource_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidateResourceConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/ValidateDataResourceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateDataResourceConfigSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::validate_data_resource_config::Request>
                        for ValidateDataResourceConfigSvc<T>
                    {
                        type Response = super::validate_data_resource_config::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::validate_data_resource_config::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).validate_data_resource_config(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidateDataResourceConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/UpgradeResourceState" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradeResourceStateSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::upgrade_resource_state::Request>
                        for UpgradeResourceStateSvc<T>
                    {
                        type Response = super::upgrade_resource_state::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::upgrade_resource_state::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upgrade_resource_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpgradeResourceStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/ConfigureProvider" => {
                    #[allow(non_camel_case_types)]
                    struct ConfigureProviderSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::configure_provider::Request>
                        for ConfigureProviderSvc<T>
                    {
                        type Response = super::configure_provider::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::configure_provider::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).configure_provider(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfigureProviderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/ReadResource" => {
                    #[allow(non_camel_case_types)]
                    struct ReadResourceSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::UnaryService<super::read_resource::Request>
                        for ReadResourceSvc<T>
                    {
                        type Response = super::read_resource::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::read_resource::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_resource(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/PlanResourceChange" => {
                    #[allow(non_camel_case_types)]
                    struct PlanResourceChangeSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::plan_resource_change::Request>
                        for PlanResourceChangeSvc<T>
                    {
                        type Response = super::plan_resource_change::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::plan_resource_change::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).plan_resource_change(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PlanResourceChangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/ApplyResourceChange" => {
                    #[allow(non_camel_case_types)]
                    struct ApplyResourceChangeSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::apply_resource_change::Request>
                        for ApplyResourceChangeSvc<T>
                    {
                        type Response = super::apply_resource_change::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::apply_resource_change::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).apply_resource_change(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ApplyResourceChangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/ImportResourceState" => {
                    #[allow(non_camel_case_types)]
                    struct ImportResourceStateSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider>
                        tonic::server::UnaryService<super::import_resource_state::Request>
                        for ImportResourceStateSvc<T>
                    {
                        type Response = super::import_resource_state::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::import_resource_state::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).import_resource_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ImportResourceStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/ReadDataSource" => {
                    #[allow(non_camel_case_types)]
                    struct ReadDataSourceSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::UnaryService<super::read_data_source::Request>
                        for ReadDataSourceSvc<T>
                    {
                        type Response = super::read_data_source::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::read_data_source::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_data_source(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadDataSourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tfplugin6.Provider/StopProvider" => {
                    #[allow(non_camel_case_types)]
                    struct StopProviderSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::UnaryService<super::stop_provider::Request>
                        for StopProviderSvc<T>
                    {
                        type Response = super::stop_provider::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::stop_provider::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_provider(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopProviderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
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
    impl<T: Provider> Clone for ProviderServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Provider> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Provider> tonic::transport::NamedService for ProviderServer<T> {
        const NAME: &'static str = "tfplugin6.Provider";
    }
}
