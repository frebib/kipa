//! Contain server-based code for communicating between two nodes.
//!
//! Servers in this file use generic socket types to read and write data from
//! sockets, and use `DataHandler` types to convert these into `Request`s and
//! `Response`s.

use error::*;
use node::Node;
use api::{RequestMessage, ResponseMessage, ResponsePayload};
use request_handler::RequestHandler;
use data_transformer::DataTransformer;

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use error_chain::ChainedError;
use std::io::Cursor;
use std::io::{Read, Write};
use std::mem::size_of;
use std::sync::Arc;

use slog::Logger;

/// The default port for server communication.
pub const DEFAULT_PORT: u16 = 10842;

/// Create a server that can listen for requests and pass onto a
/// `RequestHandler`.
pub trait SocketServer: Send + Sync {
    /// The type of the socket to use for sending/receiveing data
    type SocketType: Read + Write;

    /// Get the logger for this instance
    fn get_log(&self) -> &Logger;

    /// Handle a socket that the server has receieved wrapped in a result.
    fn handle_socket_result(
        &self,
        socket_result: Result<Self::SocketType>,
        request_handler: Arc<RequestHandler>,
        data_transformer: Arc<DataTransformer>,
    ) {
        let result = socket_result.and_then(|mut socket| {
            self.handle_socket(
                &mut socket,
                &*request_handler,
                &*data_transformer,
            )
        });

        if let Err(err) = result {
            error!(
                self.get_log(),
                "Exception when handling socket";
                "exception" => %err.display_chain());
        }
    }

    /// Handle a socket that the server has received.
    fn handle_socket(
        &self,
        socket: &mut Self::SocketType,
        request_handler: &RequestHandler,
        data_transformer: &DataTransformer,
    ) -> Result<()> {
        trace!(self.get_log(), "Reading request from socket");
        let request_data = receive_data(socket)?;

        trace!(self.get_log(), "Processing request");
        let request =
            data_transformer.bytes_to_request(&request_data.to_vec())?;

        trace!(self.get_log(), "Sending response");
        let response_payload = request_handler.receive(&request)?;
        let response = self.payload_to_response(response_payload);
        let response_data = data_transformer.response_to_bytes(&response)?;
        send_data(&response_data, socket)?;
        trace!(self.get_log(), "Sent response bytes");
        Ok(())
    }

    /// Convert a payload of a response into a response message.
    fn payload_to_response(
        &self,
        response_payload: ResponsePayload,
    ) -> ResponseMessage;

    /// Check that the request is OK to process.
    fn check_request(&self, request: &RequestMessage) -> Result<()>;
}

/// Functionality for sending requests to other KIPA servers on a socket.
pub trait SocketClient {
    /// The socket type to send/receive data from.
    type SocketType: Read + Write;

    /// Get the logger for this instance
    fn get_log(&self) -> &Logger;

    /// Create a socket to connect to the `node`.
    fn create_socket(&self, node: &Node) -> Result<Self::SocketType>;

    /// Send a request to another `Node` and get the `Response`.
    fn receive<'a>(
        &self,
        node: &Node,
        request: RequestMessage,
        data_transformer: &DataTransformer,
    ) -> Result<ResponseMessage> {
        let request_bytes = data_transformer.request_to_bytes(&request)?;

        trace!(
            self.get_log(),
            "Setting up socket";
            "node" => %node
        );
        let mut socket = self.create_socket(node)?;

        trace!(self.get_log(), "Sending request to another node");
        send_data(&request_bytes, &mut socket)?;

        trace!(self.get_log(), "Reading response from another node");
        let response_data = receive_data(&mut socket)?;

        trace!(self.get_log(), "Got response bytes");
        data_transformer.bytes_to_response(&response_data)
    }
}

/// Send data down a socket. Handles writing the length of the data.
pub fn send_data<SocketType: Write>(
    data: &Vec<u8>,
    socket: &mut SocketType,
) -> Result<()> {
    let mut len_data = vec![];
    len_data
        .write_u32::<NetworkEndian>(data.len() as u32)
        .chain_err(|| "Error on encoding length as byte array")?;
    socket
        .write(&len_data)
        .chain_err(|| "Error on writing length")?;
    socket
        .write(&data)
        .chain_err(|| "Error on writing response data")?;
    Ok(())
}

/// Receive data from a socket. Handles reading the length of the data.
pub fn receive_data<SocketType: Read>(
    socket: &mut SocketType,
) -> Result<Vec<u8>> {
    const SIZE_OF_LEN: usize = size_of::<u32>();
    let mut len_data: [u8; SIZE_OF_LEN] = [0; SIZE_OF_LEN];
    socket
        .read_exact(&mut len_data)
        .chain_err(|| "Error on reading length data")?;
    let mut cursor = Cursor::new(len_data);
    let len = cursor
        .read_u32::<NetworkEndian>()
        .chain_err(|| "Error on casting length data to u32")?;
    let mut data = vec![0 as u8; len as usize];
    socket
        .read_exact(&mut data)
        .chain_err(|| "Error on read main data")?;

    Ok(data)
}
