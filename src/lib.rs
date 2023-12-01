mod middleware;
mod model;
mod routes;
mod swagger;
mod utils;

use sea_orm::Database;

pub async fn run(db_uri: &str) {
    let db = Database::connect(db_uri).await.unwrap();

    let services = routes::create_routes(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // run our app with hyper, listening globally on port 3000
    axum::serve(listener, services).await.unwrap();
}
