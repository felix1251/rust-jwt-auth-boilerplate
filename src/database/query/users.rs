use crate::{
    models::{
        prelude::Users,
        users::{self, Model as UsersModel},
    },
    utils::app_error::{AppError, DynamicAppError},
};
use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn find_user_by_id(
    id: i32,
    db: DatabaseConnection,
) -> Result<Option<UsersModel>, AppError> {
    let user = Users::find_by_id(id).one(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            DynamicAppError::String("INTERNAL_SERVER_ERROR"),
        )
    })?;

    return Ok(user);
}

pub async fn find_user_by_email(
    email: String,
    db: DatabaseConnection,
) -> Result<Option<UsersModel>, AppError> {
    let user = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(&db)
        .await
        .map_err(|_err| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                DynamicAppError::String("INTERNAL_SERVER_ERROR"),
            )
        })?;

    return Ok(user);
}
