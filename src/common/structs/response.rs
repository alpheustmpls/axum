use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// JSON response error.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JsonResponseErrorStruct<ErrorCode = String> {
    /// Code representing the error.
    pub code: ErrorCode,
    /// Indicates where the error occurred.
    pub path: Vec<String>,
    /// Detail of the error.
    pub message: Option<String>,
}

/// Success JSON response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JsonSuccessResponse<Data = ()> {
    /// Indicates whether the response is successful or not.
    pub success: bool,
    /// Requested information for the response when `success` is `true`.
    pub data: Data,
    /// A list of errors for the response when `success` is `false`.
    #[schema(max_items = 0)]
    pub errors: Vec<JsonResponseErrorStruct>,
}

/// Failure JSON response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JsonFailureResponse<Code = String> {
    /// Indicates whether the response is successful or not.
    pub success: bool,
    /// Requested information for the response when `success` is `true`.
    pub data: Option<()>,
    /// A list of errors for the response when `success` is `false`.
    pub errors: Vec<JsonResponseErrorStruct<Code>>,
}
