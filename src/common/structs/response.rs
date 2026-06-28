use literalize::literal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

/// JSON response error.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JsonResponseErrorStruct<
    Code = String,
    Path = String,
    Message = String,
> {
    /// Code representing the error.
    pub code: Code,
    /// Indicates where the error occurred.
    pub path: Vec<Path>,
    /// Detail of the error.
    pub message: Option<Message>,
}

#[derive(Clone)]
#[literal(true)]
pub struct LiteralTrue;

/// Success JSON response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JsonSuccessResponse<Data = ()> {
    /// Indicates whether the response is successful or not.
    pub success: LiteralTrue,
    /// Requested information for the response when `success` is `true`.
    pub data: Data,
    /// A list of errors for the response when `success` is `false`.
    pub errors: Vec<Value>,
}

#[derive(Clone)]
#[literal(false)]
pub struct LiteralFalse;

/// Failure JSON response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JsonFailureResponse<Error = JsonResponseErrorStruct> {
    /// Indicates whether the response is successful or not.
    pub success: LiteralFalse,
    /// Requested information for the response when `success` is `true`.
    pub data: Option<()>,
    /// A list of errors for the response when `success` is `false`.
    pub errors: Vec<Error>,
}
