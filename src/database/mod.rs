pub mod mutation;
pub mod query;

use sea_orm::{Database, DatabaseConnection};

pub async fn init(db_uri: String) -> DatabaseConnection {
    Database::connect(db_uri).await.unwrap()
}
