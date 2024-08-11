pub mod status;
use serde::{Deserialize, Serialize};

use crate::errors::Error;

/// Request invocation info
#[derive(Debug, Serialize, Deserialize)]
pub struct InvocationInfo {
    #[serde(rename = "exec-duration-millis")]
    exec_duration_millis: Option<usize>,

    hostname: String,

    #[serde(rename = "req-id")]
    request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YmApiError {
    pub message: String,
    pub name: String,
}

impl From<YmApiError> for Error {
    fn from(error: YmApiError) -> Self {
        match error.name.as_str() {
            "session_expired" => Error::ApiError(crate::errors::ApiError::TokenExpired),
            _ => panic!("Unimplemented error: {}", error.name),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YmApiResponse<T>
{
    invocation_info: InvocationInfo,
    result: Option<T>,
    error: Option<YmApiError>,
}

impl <T> YmApiResponse<T> {
    pub fn result(&self) -> Option<&T> {
        self.result.as_ref()
    }

    pub fn error(&self) -> Option<&YmApiError> {
        self.error.as_ref()
    }

    pub fn invocation_info(&self) -> &InvocationInfo {
        &self.invocation_info
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Plus {
    has_plus: bool,
    is_tutorial_complated: bool,
}