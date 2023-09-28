// @generated
/// Wasm light client's Client state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub code_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub latest_height: ::core::option::Option<super::super::super::proto::core::client::Height>,
}
/// Wasm light client's ConsensusState
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub code_id: ::prost::alloc::vec::Vec<u8>,
    /// timestamp that corresponds to the block height in which the ConsensusState
    /// was stored.
    #[prost(uint64, tag="3")]
    pub timestamp: u64,
    /// commitment root
    #[prost(message, optional, tag="4")]
    pub root: ::core::option::Option<super::super::super::proto::core::commitment::MerkleRoot>,
}
/// Wasm light client Header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub height: ::core::option::Option<super::super::super::proto::core::client::Height>,
}
/// Wasm light client Misbehaviour
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehaviour {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Encoded file descriptor set for the `icon.lightclient.v1.wasm` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xba, 0x11, 0x0a, 0x1e, 0x69, 0x63, 0x6f, 0x6e, 0x2f, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2f, 0x76, 0x31, 0x2f, 0x77, 0x61, 0x73, 0x6d, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x18, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x77, 0x61, 0x73, 0x6d, 0x1a, 0x14, 0x67,
    0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x1b, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x30, 0x32, 0x2d, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x2f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x23, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x32, 0x33, 0x2d, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x6d, 0x65, 0x6e, 0x74, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0xa3, 0x01, 0x0a, 0x0b, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x64, 0x61, 0x74, 0x61, 0x12, 0x17, 0x0a, 0x07, 0x63, 0x6f, 0x64, 0x65, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x63, 0x6f, 0x64, 0x65, 0x49, 0x64, 0x12, 0x61, 0x0a,
    0x0d, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x48, 0x65,
    0x69, 0x67, 0x68, 0x74, 0x42, 0x1c, 0xc8, 0xde, 0x1f, 0x00, 0xf2, 0xde, 0x1f, 0x14, 0x79, 0x61,
    0x6d, 0x6c, 0x3a, 0x22, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x68, 0x65, 0x69, 0x67, 0x68,
    0x74, 0x22, 0x52, 0x0c, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x3a, 0x04, 0x88, 0xa0, 0x1f, 0x00, 0x22, 0x9d, 0x01, 0x0a, 0x0e, 0x43, 0x6f, 0x6e, 0x73, 0x65,
    0x6e, 0x73, 0x75, 0x73, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x17, 0x0a,
    0x07, 0x63, 0x6f, 0x64, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06,
    0x63, 0x6f, 0x64, 0x65, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x12, 0x3a, 0x0a, 0x04, 0x72, 0x6f, 0x6f, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x26, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x63, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x2e,
    0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x04, 0x72, 0x6f, 0x6f, 0x74,
    0x3a, 0x04, 0x88, 0xa0, 0x1f, 0x00, 0x22, 0x71, 0x0a, 0x06, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x64, 0x61, 0x74, 0x61, 0x12, 0x4d, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x48, 0x65,
    0x69, 0x67, 0x68, 0x74, 0x42, 0x15, 0xc8, 0xde, 0x1f, 0x00, 0xf2, 0xde, 0x1f, 0x0d, 0x79, 0x61,
    0x6d, 0x6c, 0x3a, 0x22, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x22, 0x52, 0x06, 0x68, 0x65, 0x69,
    0x67, 0x68, 0x74, 0x3a, 0x04, 0x88, 0xa0, 0x1f, 0x00, 0x22, 0x5b, 0x0a, 0x0c, 0x4d, 0x69, 0x73,
    0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x75, 0x72, 0x12, 0x31, 0x0a, 0x09, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x14, 0xf2, 0xde,
    0x1f, 0x10, 0x79, 0x61, 0x6d, 0x6c, 0x3a, 0x22, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69,
    0x64, 0x22, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61,
    0x3a, 0x04, 0x88, 0xa0, 0x1f, 0x00, 0x42, 0xed, 0x01, 0x0a, 0x1c, 0x63, 0x6f, 0x6d, 0x2e, 0x69,
    0x63, 0x6f, 0x6e, 0x2e, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e,
    0x76, 0x31, 0x2e, 0x77, 0x61, 0x73, 0x6d, 0x42, 0x09, 0x57, 0x61, 0x73, 0x6d, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x50, 0x01, 0x5a, 0x3e, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d,
    0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x69, 0x62, 0x63, 0x2d, 0x67, 0x6f, 0x2f, 0x76,
    0x37, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x73, 0x2f, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x2d,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x2f, 0x30, 0x38, 0x2d, 0x77, 0x61, 0x73, 0x6d, 0x3b,
    0x77, 0x61, 0x73, 0x6d, 0xa2, 0x02, 0x04, 0x49, 0x4c, 0x56, 0x57, 0xaa, 0x02, 0x18, 0x49, 0x63,
    0x6f, 0x6e, 0x2e, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x56,
    0x31, 0x2e, 0x57, 0x61, 0x73, 0x6d, 0xca, 0x02, 0x18, 0x49, 0x63, 0x6f, 0x6e, 0x5c, 0x4c, 0x69,
    0x67, 0x68, 0x74, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5c, 0x56, 0x31, 0x5c, 0x57, 0x61, 0x73,
    0x6d, 0xe2, 0x02, 0x24, 0x49, 0x63, 0x6f, 0x6e, 0x5c, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5c, 0x56, 0x31, 0x5c, 0x57, 0x61, 0x73, 0x6d, 0x5c, 0x47, 0x50, 0x42,
    0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1b, 0x49, 0x63, 0x6f, 0x6e, 0x3a,
    0x3a, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x3a, 0x3a, 0x56, 0x31,
    0x3a, 0x3a, 0x57, 0x61, 0x73, 0x6d, 0x4a, 0xfc, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x2f,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x1e,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x25, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x06, 0x00, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x07, 0x00,
    0x23, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x55, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x09, 0x00, 0x55, 0x0a, 0x2e, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x00,
    0x12, 0x01, 0x1a, 0x22, 0x20, 0x57, 0x61, 0x73, 0x6d, 0x20, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x27, 0x73, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c,
    0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x0d, 0x02, 0x32, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x07, 0x81, 0xf4, 0x03, 0x12, 0x03, 0x0d, 0x02, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0e, 0x1c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0e, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x1c, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0x10, 0x02, 0x11, 0x56, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x10, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x10, 0x20, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x10, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x11, 0x06,
    0x55, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x11,
    0x07, 0x23, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xee, 0xfb, 0x03, 0x12, 0x03,
    0x11, 0x25, 0x54, 0x0a, 0x30, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00, 0x1f, 0x01, 0x1a,
    0x24, 0x20, 0x57, 0x61, 0x73, 0x6d, 0x20, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x27, 0x73, 0x20, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x15, 0x08,
    0x16, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x07, 0x12, 0x03, 0x16, 0x02, 0x2d, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x01, 0x07, 0x81, 0xf4, 0x03, 0x12, 0x03, 0x16, 0x02, 0x2d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x17, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x17, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x17, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x02, 0x07, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x27, 0x28, 0x0a, 0x66, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x1c, 0x02, 0x17, 0x1a, 0x59, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x20, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x0a, 0x20, 0x77, 0x61, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x09, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x15, 0x16, 0x0a, 0x1e, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x02, 0x31, 0x1a, 0x11, 0x20, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x6f, 0x6f, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1e, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x28, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x1e, 0x2f, 0x30, 0x0a, 0x26, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x22, 0x00,
    0x27, 0x01, 0x1a, 0x1a, 0x20, 0x57, 0x61, 0x73, 0x6d, 0x20, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x22, 0x08, 0x0e, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x07, 0x12, 0x03, 0x23, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x07, 0x81, 0xf4, 0x03,
    0x12, 0x03, 0x23, 0x02, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x25,
    0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x25, 0x02, 0x07,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x1c, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x02, 0x74, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x26, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x26, 0x20, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x26, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x08, 0x12, 0x03, 0x26, 0x2b,
    0x73, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x26,
    0x2c, 0x48, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x01, 0x08, 0xee, 0xfb, 0x03, 0x12, 0x03,
    0x26, 0x4a, 0x72, 0x0a, 0x2c, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2a, 0x00, 0x2f, 0x01, 0x1a,
    0x20, 0x20, 0x57, 0x61, 0x73, 0x6d, 0x20, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x20, 0x4d, 0x69, 0x73, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x75, 0x72,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x14, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x07, 0x12, 0x03, 0x2b, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x07,
    0x81, 0xf4, 0x03, 0x12, 0x03, 0x2b, 0x02, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x2d, 0x02, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x2d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x09,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x15, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2d, 0x17, 0x44, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x03, 0x02, 0x00, 0x08, 0xee, 0xfb, 0x03, 0x12, 0x03, 0x2d, 0x18, 0x43, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x2e, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x2e, 0x15, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("icon.lightclient.v1.wasm.serde.rs");
// @@protoc_insertion_point(module)