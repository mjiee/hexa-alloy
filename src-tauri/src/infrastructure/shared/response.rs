use serde::{Deserialize, Serialize};

/// Represents a generic response structure that can be used to wrap the results of API calls or operations.
///
/// The `Response` struct is designed to standardize the format of responses across the application.
/// It includes a status code, a message, and the actual data payload.
///
/// # Type Parameters
/// - `T`: A generic type parameter representing the type of the data payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    /// The status code of the response. A numerical value indicating the outcome of the operation.
    code: i32,
    /// A human-readable message providing additional information about the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    /// The actual data payload returned by the operation. The type is generic and can be any type.
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
}

impl<T> Response<T> {
    /// Creates a new `Response` instance with the provided status code, message, and data.
    ///
    /// # Arguments
    /// - `code`: The status code of the response.
    /// - `message`: A human-readable message describing the response.
    /// - `result`: The actual data payload of the response.
    ///
    /// # Returns
    /// A new `Response` instance with the specified values.
    pub fn new(code: i32, message: Option<String>, result: T) -> Self {
        Response {
            code,
            message,
            result: Option::Some(result),
        }
    }

    /// Creates a new `Response` instance representing a successful operation.
    ///
    /// This method sets the status code to `0` and the message to `"ok"`.
    ///
    /// # Arguments
    /// - `result`: The actual data payload of the successful response.
    ///
    /// # Returns
    /// A new `Response` instance with the status code `0`, message `"ok"`, and the provided data.
    pub fn ok(result: T) -> Self {
        Response::new(0, None, result)
    }
}
