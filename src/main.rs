use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use project::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_uri = format!("{}", dotenv!("DATABASE_URL"));
    run(db_uri).await
}
