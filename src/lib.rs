mod routes;
mod utils;

pub async fn run() {
    // build our application with a single route
    let services = routes::create_routes();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, services).await.unwrap();
}
