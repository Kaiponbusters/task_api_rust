use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use crate::repo::RepoError;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    NotFound,

    #[error("bad request : {0}")]
    BadRequest(String),

    #[error("internal error : {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorBody {
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}
impl From<RepoError> for ApiError {
    fn from(err: RepoError) -> Self {
        //TODO : ApiError::Internal()にする
        ApiError::Internal(err.to_string())
    }
}



pub fn validate_title(title: &str) -> Result<(), ApiError> {
    let t = title.trim();
    if t.is_empty() {
        return Err(ApiError::BadRequest("title is empty".to_string()));
    }
    if t.chars().count() > 100 {
        return Err(ApiError::BadRequest("title is too long".to_string()));
    }
    Ok(())
}
