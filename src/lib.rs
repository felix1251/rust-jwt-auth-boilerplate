mod routes;
mod utils;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use routes::{examples, home::home};

// Custom not found route fallback
async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "ROUTE_NOT_FOUND")
}

pub async fn run() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .nest(
            "/examples",
            Router::new()
                .route("/test_json", post(examples::test_json))
                .route("/path_vars/:id", get(examples::path_vars))
                .route("/query_params", get(examples::query_params))
                .route("/headers", get(examples::headers)),
        )
        .fallback(fallback);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
