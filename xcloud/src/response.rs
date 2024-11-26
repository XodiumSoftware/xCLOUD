use serde::Serialize;
use std::borrow::Cow;

/// A struct representing the response of an API request.
#[derive(Serialize, Debug, Clone)]
pub struct ApiResponse<'a, T> {
    /// The status of the response, e.g., "success" or "error".
    status: &'static str,
    /// The message associated with the response.
    message: Cow<'a, str>,
    /// The optional data payload of the response.
    data: Option<T>,
}

/// Implementation of the `ApiResponse` struct.
impl<'a, T> ApiResponse<'a, T> {
    /// Creates a success response.
    ///
    /// # Arguments
    ///
    /// * `message` - The success message.
    /// * `data` - The optional data payload.
    ///
    /// # Returns
    ///
    /// * `ApiResponse` - The success response.
    pub fn success<S>(message: S, data: Option<T>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            status: "success",
            message: message.into(),
            data,
        }
    }

    /// Creates an error response.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message.
    ///
    /// # Returns
    ///
    /// * `ApiResponse` - The error response.
    pub fn error<S>(message: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            status: "error",
            message: message.into(),
            data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response: ApiResponse<&str> =
            ApiResponse::success("Operation successful", Some("data"));
        assert_eq!(response.status, "success");
        assert_eq!(response.message, "Operation successful");
        assert_eq!(response.data, Some("data"));
    }

    #[test]
    fn test_error_response() {
        let response: ApiResponse<()> = ApiResponse::error("Operation failed");
        assert_eq!(response.status, "error");
        assert_eq!(response.message, "Operation failed");
        assert_eq!(response.data, None);
    }
}
