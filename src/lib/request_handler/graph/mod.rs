//! Implement `RequestHandler` using graph based searches through KIPA net.

mod search;
mod neighbours_store;
mod key_space;

use error::*;
use server::Client;
use key::Key;
use node::Node;
use request_handler::graph::neighbours_store::NeighboursStore;
use request_handler::graph::key_space::{sort_key_relative, KeySpace};
use request_handler::graph::search::{GraphSearch, SearchCallbackReturn};
use api::{MessageSender, RequestMessage, RequestPayload, ResponsePayload};
use request_handler::RequestHandler;

use std::sync::{Arc, Mutex};

/// The default size of the neighbours store
pub const DEFAULT_NEIGHBOURS_SIZE: usize = 3;

/// The default dimension size for key space
pub const DEFAULT_KEY_SPACE_SIZE: usize = 2;

/// The default bredth of the search to use when connecting
pub const DEFAULT_CONNECT_SEARCH_SIZE: usize = 3;

/// Contains graph search information.
pub struct GraphRequestHandler {
    key: Key,
    neighbours_store: Arc<Mutex<NeighboursStore>>,
    graph_search: Arc<GraphSearch>,
    key_space_size: usize,
}

impl GraphRequestHandler {
    /// Create a new graph request handler.
    ///
    /// - `key` is the key for the local node.
    /// - `remote_server` is used for communicating with other nodes.
    /// - `initial_node` is the initial other node in KIPA network.
    pub fn new(
        key: Key,
        remote_server: Arc<Client>,
        neighbours_size: usize,
        key_space_size: usize,
    ) -> Self {
        let remote_server_clone = remote_server.clone();
        let graph_search = GraphSearch::new(Arc::new(move |n, k: &Key| {
            let response = remote_server_clone
                .receive(n, RequestPayload::QueryRequest(k.clone()))?;

            match response.payload {
                ResponsePayload::QueryResponse(ref nodes) => Ok(nodes.clone()),
                _ => Err(ErrorKind::ResponseError(
                    "Incorrect response for query request".into(),
                ).into()),
            }
        }));

        let neighbours_store =
            NeighboursStore::new(key.clone(), neighbours_size, key_space_size);

        GraphRequestHandler {
            key: key,
            neighbours_store: Arc::new(Mutex::new(neighbours_store)),
            graph_search: Arc::new(graph_search),
            key_space_size: key_space_size,
        }
    }

    fn search(&self, key: &Key) -> Result<Option<Node>> {
        let initial_nodes = self.neighbours_store.lock().unwrap().get_all();
        let callback_key = key.clone();
        let found_callback = move |n: &Node| {
            trace!("Found node when searching: {}", n);
            if n.key == callback_key {
                Ok(SearchCallbackReturn::Return(n.clone()))
            } else {
                Ok(SearchCallbackReturn::Continue())
            }
        };

        self.graph_search.search(
            &key,
            initial_nodes,
            Arc::new(found_callback),
            Arc::new(|n| {
                trace!("Found node when searching: {}", n);
                Ok(SearchCallbackReturn::Continue())
           }),
        )
    }

    fn connect(&self, node: &Node) -> Result<()> {
        // Continue the graph search looking for ourselves, until the `n`
        // closest nodes to ourselves have also been explored.

        // List of tuples of the `n` closest nodes, where first is the node,
        // and second is a boolean telling whether it has been explored.
        let n_closest: Arc<Mutex<Vec<(Node, bool)>>> = Arc::new(Mutex::new(
            Vec::with_capacity(DEFAULT_CONNECT_SEARCH_SIZE),
        ));

        let local_key_space =
            Arc::new(KeySpace::from_key(&self.key, self.key_space_size));

        let found_n_closest = n_closest.clone();
        let found_key_space_size = self.key_space_size.clone();
        let found_neighbours_store = self.neighbours_store.clone();
        let found_callback = move |n: &Node| {
            trace!("Found node when connecting: {}", n);
            // Consider the connected node as a candidate
            found_neighbours_store
                .lock()
                .expect("Failed to lock found_neighbours_store")
                .consider_candidate(n);

            // Add the new node to `n_closest`, sort it, and remove the last
            let mut n_closest_local = found_n_closest
                .lock()
                .expect("Failed to lock found_n_closest");
            n_closest_local.push((n.clone(), false));
            sort_key_relative(
                &mut n_closest_local,
                &|&(ref n, _)| KeySpace::from_key(&n.key, found_key_space_size),
                &local_key_space,
            );
            n_closest_local.pop();

            Ok(SearchCallbackReturn::Continue())
        };

        let explored_n_closest = n_closest.clone();
        let explored_callback = move |n: &Node| {
            trace!("Explored node when connecting: {}", n);

            // Set the `n_closest` value to explored
            let mut n_closest_local = explored_n_closest
                .lock()
                .expect("Failed to lock explored_n_closest");
            for tuple in &mut *n_closest_local {
                if n == &tuple.0 {
                    tuple.1 = true;
                }
            }

            // Check if all of the `n_closest` has been explored
            let all_explored = n_closest_local.iter().all(&|&(_, ref e)| *e);

            if all_explored {
                Ok(SearchCallbackReturn::Return(()))
            } else {
                Ok(SearchCallbackReturn::Continue())
            }
        };

        self.graph_search.search(
            &self.key,
            vec![node.clone()],
            Arc::new(found_callback),
            Arc::new(explored_callback),
        )?;
        Ok(())
    }
}

impl RequestHandler for GraphRequestHandler {
    fn receive(&self, request: &RequestMessage) -> Result<ResponsePayload> {
        info!("Received request from {}", request.sender);

        match &request.sender {
            &MessageSender::Node(ref n) =>
                self.neighbours_store.lock().unwrap().consider_candidate(&n),
            &MessageSender::Cli() => {},
        };

        match &request.payload {
            &RequestPayload::QueryRequest(ref key) => {
                trace!("Received query request");
                Ok(ResponsePayload::QueryResponse(
                    self.neighbours_store.lock().unwrap().get_n_closest(key, 1),
                ))
            }
            &RequestPayload::SearchRequest(ref key) => {
                trace!("Received search request for key {}", key);
                Ok(ResponsePayload::SearchResponse(self.search(&key)?))
            }
            &RequestPayload::ConnectRequest(ref node) => {
                trace!("Received connect request for node {}", node);
                self.connect(node)?;
                Ok(ResponsePayload::ConnectResponse())
            }
        }
    }
}
