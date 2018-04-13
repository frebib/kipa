//! Serialize and deserialize requests and responses for communication over
//! the wire between KIPA nodes/interfaces.

use error::*;
use api::{RequestMessage, ResponseMessage};

#[cfg(feature = "use-protobuf")]
pub mod protobuf;
#[cfg(feature = "use-protobuf")]
mod proto_api;

/// Implementors must be able to convert `Request`s and `Response`s to and from
/// bytes.
pub trait DataTransformer: Send + Sync {
    /// Convert a `Request` to bytes.
    fn request_to_bytes(&self, request: &RequestMessage) -> Result<Vec<u8>>;

    /// Convert a bytes to a `Request`.
    fn bytes_to_request(&self, data: &Vec<u8>) -> Result<RequestMessage>;

    /// Convert a `Response` to bytes.
    fn response_to_bytes(&self, response: &ResponseMessage) -> Result<Vec<u8>>;

    /// Convert a bytes to a `Response`.
    fn bytes_to_response(&self, data: &Vec<u8>) -> Result<ResponseMessage>;
}
