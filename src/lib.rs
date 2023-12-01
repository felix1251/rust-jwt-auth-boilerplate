mod middleware;
mod routes;
mod swagger;
mod utils;

pub async fn run() {
    let services = routes::create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // run our app with hyper, listening globally on port 3000
    axum::serve(listener, services).await.unwrap();
}
