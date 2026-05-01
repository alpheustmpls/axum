#![allow(dead_code)]

use axum::http::StatusCode;
use jder_axum::response::{
    Response,
    json::{CreateJsonResponse, JsonResponseError},
};

/// Service error.
#[derive(Debug, Clone)]
pub struct ServiceError {
    pub status: StatusCode,
    pub code: String,
    pub path: Vec<String>,
    pub message: Option<String>,
}

impl ServiceError {
    /// Create a new ServiceError.
    pub fn new() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "unknown".to_string(),
            path: Vec::new(),
            message: None,
        }
    }

    /// Set status code.
    pub fn status(
        mut self,
        status: StatusCode,
    ) -> Self {
        self.status = status;

        self
    }

    /// Set error code.
    pub fn code<S: Into<String>>(
        mut self,
        code: S,
    ) -> Self {
        self.code = code.into();

        self
    }

    /// Set error path.
    pub fn path<P, S>(
        mut self,
        path: P,
    ) -> Self
    where
        P: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.path = path.into_iter().map(|s| s.into()).collect();

        self
    }

    /// Set error message.
    pub fn message<S: Into<String>>(
        mut self,
        message: S,
    ) -> Self {
        self.message = Some(message.into());

        self
    }

    /// Transform into a JDER failure response.
    pub fn into_failure_response(self) -> Response {
        let mut err: JsonResponseError =
            JsonResponseError::new().code(self.code).path(self.path);

        if let Some(message) = self.message {
            err = err.message(message);
        }

        CreateJsonResponse::failure()
            .status(self.status)
            .add_error(err)
            .create()
    }
}

impl Default for ServiceError {
    fn default() -> Self {
        Self::new()
    }
}
