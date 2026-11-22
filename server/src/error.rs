use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, serde::Serialize)]
pub struct ErrorEnvelope {
    pub error: ErrorDetail,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorDetail {
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    RangeNotSatisfiable(String),
    Unsupported(&'static str),
    Internal(anyhow::Error),
}

impl AppError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn range_not_satisfiable(msg: impl Into<String>) -> Self {
        Self::RangeNotSatisfiable(msg.into())
    }

    pub fn unsupported(feature: &'static str) -> Self {
        Self::Unsupported(feature)
    }

    pub fn internal(err: impl Into<anyhow::Error>) -> Self {
        Self::Internal(err.into())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::BadRequest(msg) => write!(f, "Bad request: {msg}"),
            Self::RangeNotSatisfiable(msg) => write!(f, "Range not satisfiable: {msg}"),
            Self::Unsupported(feature) => write!(f, "Unsupported feature: {feature}"),
            Self::Internal(err) => write!(f, "Internal error: {err}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Internal(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::RangeNotSatisfiable(msg) => (StatusCode::RANGE_NOT_SATISFIABLE, msg),
            Self::Unsupported(feature) => {
                tracing::warn!(feature, "Stremio feature is not implemented");
                (
                    StatusCode::NOT_IMPLEMENTED,
                    format!("{feature} is not implemented"),
                )
            }
            Self::Internal(err) => {
                tracing::error!(error = ?err, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (
            status,
            Json(ErrorEnvelope {
                error: ErrorDetail { message },
            }),
        )
            .into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.into())
    }
}

pub type AppResult<T = Response> = Result<T, AppError>;
