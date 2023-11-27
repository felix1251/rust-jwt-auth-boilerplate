use axum::{routing::get, Router};

mod routes;
mod utils;

pub async fn run() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "this is a health check" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
