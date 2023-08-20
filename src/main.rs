use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health_check", get(|| async { }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
