pub mod api;
pub mod handler;
pub mod model;
pub mod routes;

use crate::model::Owner;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

pub type SharedState = Arc<RwLock<Vec<Owner>>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main () {
    let owners = handler::load_owners();
    let state = Arc::new(RwLock::new(owners));
    let app = routes::app_routes().with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Multi-threaded server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();

}
