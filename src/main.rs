use std::{env, net::SocketAddr};
mod controllers;
mod middleware;
mod models;
mod mongo;
mod router;
mod utils;

#[tokio::main]
async fn main() {
    let db = mongo::database().await;
    let router = router::root::root_router(db);
    let port = env::var("PORT").unwrap_or(String::from("8000"));
    let addr = ["0.0.0.0:", &port].concat();
    let server: SocketAddr = addr.parse().expect("Could not parse socket address");
    if (axum::Server::bind(&server)
        .serve(router.into_make_service())
        .await)
        .is_err()
    {
        panic!("Could not start server")
    }
}
