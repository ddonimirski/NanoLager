use axum::{Router, routing::{get, post, put, delete}};
use axum_server::Server;
use std::net::SocketAddr;

mod routes;
use routes::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/companies", get(get_companies).post(create_company))
        .route("/companies/:id", get(get_company).put(update_company).delete(delete_company))
        .route("/customers", get(get_customers).post(create_customer))
        .route("/customers/:id", get(get_customer).put(update_customer).delete(delete_customer));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🔌 storage-service running on http://{}", addr);
    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
