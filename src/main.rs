use dotenvy::dotenv;
use project::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await
}
