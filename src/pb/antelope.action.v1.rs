// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<ActionEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionEvent {
    /// transaction
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub action: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="5")]
    pub data: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
