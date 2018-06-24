//! Implementation of servers using TCP sockets

use address::Address;
use api::{ApiVisibility, RequestMessage, ResponseMessage};
use data_transformer::DataTransformer;
use error::*;
use message_handler::MessageHandler;
use node::Node;
use server::{Client, Server};
use socket_server::{SocketClient, SocketHandler, SocketServer};

use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use slog::Logger;

/// Server that listens for global requests on a specified TCP socket
#[derive(Clone)]
pub struct TcpGlobalServer {
    message_handler: Arc<MessageHandler>,
    data_transformer: Arc<DataTransformer>,
    local_node: Node,
    log: Logger,
}

impl TcpGlobalServer {
    #[allow(missing_docs)]
    pub fn new(
        message_handler: Arc<MessageHandler>,
        data_transformer: Arc<DataTransformer>,
        local_node: Node,
        log: Logger,
    ) -> Self
    {
        TcpGlobalServer {
            message_handler,
            data_transformer,
            local_node,
            log,
        }
    }
}

impl Server for TcpGlobalServer {
    fn start(&self) -> Result<thread::JoinHandle<()>> {
        let listener = TcpListener::bind(SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
            self.local_node.address.port,
        )).chain_err(|| "Error on bind to TCP socket")?;
        info!(
            self.log,
            "Started listening for TCP connections";
            "address" => %self.local_node.address
        );

        // See `kipa_lib::server` for relevent TODO
        let arc_self = Arc::new(self.clone());
        let join_handle = thread::spawn(move || {
            listener.incoming().for_each(move |socket| {
                let spawn_self = arc_self.clone();
                thread::spawn(move || {
                    spawn_self.handle_socket_result(
                        socket.chain_err(|| "Failed to create socket"),
                        spawn_self.message_handler.clone(),
                        spawn_self.data_transformer.clone(),
                    )
                });
            });
        });

        Ok(join_handle)
    }
}

impl SocketHandler for TcpGlobalServer {
    type SocketType = TcpStream;

    fn set_socket_timeout(
        &self,
        socket: &mut TcpStream,
        timeout: Option<Duration>,
    ) -> Result<()>
    {
        socket
            .set_read_timeout(timeout)
            .chain_err(|| "Error on setting read timeout on TCP socket")?;
        socket
            .set_write_timeout(timeout)
            .chain_err(|| "Error on setting write timeout on TCP socket")?;
        Ok(())
    }

    fn get_socket_peer_address(
        &self,
        socket: &Self::SocketType,
    ) -> Option<Address>
    {
        socket
            .peer_addr()
            .chain_err(|| "Error on getting peer address")
            .and_then(|s| Address::from_socket_addr(&s))
            .ok()
    }
}

impl SocketServer for TcpGlobalServer {
    fn get_log(&self) -> &Logger { &self.log }

    fn check_request(&self, request: &RequestMessage) -> Result<()> {
        if !request.payload.is_visible(&ApiVisibility::Global()) {
            Err(ErrorKind::RequestError(
                "Request is not globally available".into(),
            ).into())
        } else {
            Ok(())
        }
    }
}

/// Implementation of sending global requests to TCP servers
pub struct TcpGlobalClient {
    data_transformer: Arc<DataTransformer>,
    log: Logger,
}

impl TcpGlobalClient {
    /// Create a new sender, which uses a `DataTransformer` to serialize packets
    /// before going on the line
    pub fn new(data_transformer: Arc<DataTransformer>, log: Logger) -> Self {
        TcpGlobalClient {
            data_transformer,
            log,
        }
    }
}

impl SocketHandler for TcpGlobalClient {
    type SocketType = TcpStream;

    fn set_socket_timeout(
        &self,
        socket: &mut TcpStream,
        timeout: Option<Duration>,
    ) -> Result<()>
    {
        socket
            .set_read_timeout(timeout)
            .chain_err(|| "Error on setting read timeout on TCP socket")?;
        socket
            .set_write_timeout(timeout)
            .chain_err(|| "Error on setting write timeout on TCP socket")?;
        Ok(())
    }

    fn get_socket_peer_address(
        &self,
        socket: &Self::SocketType,
    ) -> Option<Address>
    {
        socket
            .peer_addr()
            .chain_err(|| "Error on getting peer address")
            .and_then(|s| Address::from_socket_addr(&s))
            .ok()
    }
}

impl SocketClient for TcpGlobalClient {
    fn get_log(&self) -> &Logger { &self.log }

    fn create_socket(
        &self,
        node: &Node,
        timeout: Duration,
    ) -> Result<TcpStream>
    {
        TcpStream::connect_timeout(&node.address.to_socket_addr()?, timeout)
            .chain_err(|| {
                format!("Error on trying to connect to node {}", node)
            })
    }
}

impl Client for TcpGlobalClient {
    fn send(
        &self,
        node: &Node,
        request: RequestMessage,
        timeout: Duration,
    ) -> Result<ResponseMessage>
    {
        SocketClient::send(
            self,
            node,
            request,
            &*self.data_transformer,
            timeout,
        )
    }
}
