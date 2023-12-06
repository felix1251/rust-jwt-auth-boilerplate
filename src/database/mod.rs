use sea_orm::{Database, DatabaseConnection};

pub async fn init(db_uri: &str) -> DatabaseConnection {
    Database::connect(db_uri).await.unwrap()
}
