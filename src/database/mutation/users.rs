use crate::{
    models::users::ActiveModel as UserActiveModel,
    utils::{
        app_error::{AppError, DynamicErrorType},
        password::hash_password,
    },
};
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub fullname: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    payload: CreateUserPayload,
    db: DatabaseConnection,
) -> Result<UserActiveModel, AppError> {
    let new_user = UserActiveModel {
        uuid: Set(Uuid::new_v4()),
        fullname: Set(payload.fullname),
        email: Set(payload.email),
        encrypted_password: Set(hash_password(payload.password)?),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|err| match err {
        sea_orm::DbErr::Query(_err) => AppError::new(
            StatusCode::CONFLICT,
            DynamicErrorType::String("USER_EXIST_OR_INVALID".to_string()),
        ),
        _else => AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            DynamicErrorType::String("INTERNAL_SERVER_ERROR".to_string()),
        ),
    })?;

    return Ok(new_user);
}
