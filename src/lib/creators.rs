//! Functons for creating various aspects of the kipa_lib.
//!
//! These depend on features and conditional compilation.

use data_transformer::DataTransformer;
use error::*;
use global_server::{GlobalReceiveServer, GlobalSendServer};
use gpg_key::GpgKeyHandler;
use local_server::{LocalReceiveServer, LocalSendServer};
use request_handler::RequestHandler;

use std::sync::Arc;
use clap;

cfg_if! {
    if #[cfg(feature = "use-protobuf")] {
        use data_transformer::protobuf::ProtobufDataTransformer;

        /// Create a `DataTransformer`
        pub fn create_data_transformer() -> Result<Arc<DataTransformer>> {
            Ok(Arc::new(ProtobufDataTransformer{}))
        }
    } else {
        #[allow(missing_docs)]
        pub fn create_data_transformer() -> Result<Arc<DataTransformer>> {
            Err(ErrorKind::ConfigError(
                "A data transformer feature was not selected".into()).into())
        }
    }
}

cfg_if! {
    if #[cfg(feature = "use-tcp")] {
        use global_server::tcp::{
            TcpGlobalReceiveServer, TcpGlobalSendServer};
        use server::DEFAULT_PORT;

        /// Create a `GlobalSendServer`
        pub fn create_global_send_server(
                data_transformer: Arc<DataTransformer>) ->
                Result<Arc<GlobalSendServer>> {
            Ok(Arc::new(TcpGlobalSendServer::new(data_transformer)))
        }

        /// Create a `GlobalRecieveServer`
        pub fn create_global_receive_server(
                request_handler: Arc<RequestHandler>,
                data_transformer: Arc<DataTransformer>,
                args: &clap::ArgMatches) -> Result<Box<GlobalReceiveServer>> {
            let port = args.value_of("port")
                .unwrap_or(&DEFAULT_PORT.to_string())
                .parse::<u16>().chain_err(|| "")?;
            Ok(Box::new(TcpGlobalReceiveServer::new(
                request_handler, data_transformer.clone(), port)?))
        }
    } else {
        #[allow(missing_docs)]
        pub fn create_send_server(
                data_transformer: Arc<DataTransformer>) ->
                Result<Arc<GlobalSendServer>> {
            Err(ErrorKind::ConfigError(
                "A server feature was not selected".into()).into())
        }

        #[allow(missing_docs)]
        pub fn create_receive_server(
                request_handler: Arc<RequestHandler>,
                data_transformer: Arc<DataTransformer>,
                args: &clap::ArgMatches) -> Result<Box<GlobalReceiveServer>> {
            Err(ErrorKind::ConfigError(
                "A server feature was not selected".into()).into())
        }
    }
}

cfg_if! {
    if #[cfg(feature = "use-unix-socket")] {
        use local_server::unix_socket::{
            UnixSocketLocalReceiveServer,
            UnixSocketLocalSendServer,
            DEFAULT_UNIX_SOCKET_PATH};

        /// Create a `LocalReceiveServer`
        pub fn create_local_receive_server(
                request_handler: Arc<RequestHandler>,
                data_transformer: Arc<DataTransformer>,
                args: &clap::ArgMatches) -> Result<Box<LocalReceiveServer>> {
            let socket_path = args.value_of("socket_path")
                .unwrap_or(DEFAULT_UNIX_SOCKET_PATH);
            Ok(Box::new(UnixSocketLocalReceiveServer::new(
                request_handler,
                data_transformer,
                &String::from(socket_path))?))
        }

        /// Create a `LocalSendServer`
        pub fn create_local_send_server(
                data_transformer: Arc<DataTransformer>,
                args: &clap::ArgMatches) -> Result<Arc<LocalSendServer>> {
            let socket_path = args.value_of("socket_path")
                .unwrap_or(DEFAULT_UNIX_SOCKET_PATH);
            Ok(Arc::new(UnixSocketLocalSendServer::new(
                data_transformer, &String::from(socket_path))))
        }
    } else {
        #[allow(missing_docs)]
        pub fn create_local_receive_server(
                request_handler: Arc<RequestHandler>,
                data_transformer: Arc<DataTransformer>,
                args: &clap::ArgMatches) -> Result<Box<LocalReceiveServer>> {
            Err(ErrorKind::ConfigError(
                "A local server feature was not selected".into()).into())
        }
        #[allow(missing_docs)]
        pub fn create_local_send_server(
                data_transformer: Arc<DataTransformer>,
                args: &clap::ArgMatches) -> Result<Arc<LocalSendServer>> {
            Err(ErrorKind::ConfigError(
                "A local server feature was not selected".into()).into())
        }
    }
}

cfg_if! {
    if #[cfg(feature = "use-graph")] {
        use request_handler::graph::{
            GraphRequestHandler,
            DEFAULT_NEIGHBOURS_SIZE,
            DEFAULT_KEY_SPACE_SIZE};

        /// Create a `RequestHandler`
        pub fn create_request_handler(
                gpg_key_handler: &mut GpgKeyHandler,
                remote_server: Arc<GlobalSendServer>,
                args: &clap::ArgMatches) -> Result<Arc<RequestHandler>> {

            // Get local key
            let local_key = gpg_key_handler.get_key(
                String::from(args.value_of("key_id").unwrap()))?;

            let neighbours_size = args.value_of("neighbours_size")
                .unwrap_or(&DEFAULT_NEIGHBOURS_SIZE.to_string())
                .parse::<usize>()
                .chain_err(|| "Error on parsing neighbour size")?;

            let key_space_size = args.value_of("key_space_size")
                .unwrap_or(&DEFAULT_KEY_SPACE_SIZE.to_string())
                .parse::<usize>()
                .chain_err(|| "Error on parsing key space size")?;

            Ok(Arc::new(GraphRequestHandler::new(
                local_key, remote_server, neighbours_size, key_space_size)))
        }
    } else if #[cfg(feature = "use-black-hole")] {
        use request_handler::black_hole::BlackHoleRequestHandler;

        pub fn create_request_handler(
                gpg_key_handler: &mut GpgKeyHandler,
                remote_server: Arc<GlobalSendServer>,
                args: &clap::ArgMatches) -> Result<Arc<RequestHandler>> {
            Ok(Arc::new(BlackHoleRequestHandler::new()))
        }
    } else {
        #[allow(missing_docs)]
        pub fn create_request_handler(
                gpg_key_handler: &mut GpgKeyHandler,
                remote_server: Arc<GlobalSendServer>,
                args: &clap::ArgMatches) -> Result<Arc<RequestHandler>> {
            Err(ErrorKind::ConfigError(
                "A request handler feature was not selected".into()).into())
        }
    }
}