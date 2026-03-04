use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Conflict(String),
    Db(sqlx::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::Db(e)
    }
}

#[derive(Serialize)]
struct ErrBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(ErrBody { error: msg })).into_response()
            }
            ApiError::Conflict(msg) => {
                (StatusCode::CONFLICT, Json(ErrBody { error: msg })).into_response()
            }
            ApiError::Db(e) => {
                tracing::error!("db error: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrBody {
                        error: "internal server error".into(),
                    }),
                )
                    .into_response()
            }
        }
    }
}
