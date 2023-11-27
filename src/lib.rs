mod routes;
mod utils;

use axum::{routing::get, Router};
use routes::home::home;

pub async fn run() {
    // build our application with a single route
    let app = Router::new().route("/", get(home));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
