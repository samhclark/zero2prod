use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() {
    // let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Must be bound...");
    run(listener).await.unwrap();
}
