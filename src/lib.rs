use std::net::TcpListener;

use axum::{
    routing::{get, IntoMakeService},
    Router,
};
use hyper::server::conn::AddrIncoming;

// No longer `async`
// Returns longer Server type. Need `hyper` to write these types
/// # Panics
/// TODO    
#[must_use]
pub fn run(address: TcpListener) -> hyper::Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new().route("/health_check", get(|| async {}));

    axum::Server::from_tcp(address)
        .expect("A bound TCP address is required")
        .serve(app.into_make_service())
}
