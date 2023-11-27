mod routes;
mod utils;

use routes::create_routes;

pub async fn run() {
    // build our application with a single route
    let services = create_routes();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, services).await.unwrap();
}
