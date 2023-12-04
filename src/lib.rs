mod middleware;
mod models;
mod routes;
mod swagger;
mod utils;

use sea_orm::Database;

pub async fn run(db_uri: &str) {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::debug!("Connecting to DB");
    let db = Database::connect(db_uri).await.unwrap();
    tracing::debug!("DB Connected");

    let services = routes::create_routes(db).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let local_address = listener.local_addr().unwrap();

    tracing::debug!("listening on http://{}", local_address);
    tracing::debug!("API docs on http://{}/api/docs", local_address);

    // run our app with hyper, listening globally on port 3000
    axum::serve(listener, services).await.unwrap();
}
