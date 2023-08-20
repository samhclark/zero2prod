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
pub fn run() -> hyper::Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new().route("/health_check", get(|| async {}));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(app.into_make_service())
}
