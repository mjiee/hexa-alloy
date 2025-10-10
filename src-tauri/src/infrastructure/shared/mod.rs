use crate::infrastructure::error;

pub mod config_key;
pub mod pagination;
pub mod response;
pub mod result;

pub use pagination::Pagination;
pub use response::Response;
pub use result::QueryResult;

/// A type alias representing the result of an operation that can either succeed with a `Response<T>`
/// or fail with a `BasicError`.
///
/// - `'a`: A lifetime parameter, used to manage the lifetime of references within `BasicError`.
/// - `T`: A generic type parameter, allowing this result type to be used with different data types.
pub type RespResult<'a, T> = core::result::Result<Response<T>, error::BasicError<'a>>;

/// Creates a `RespResult` instance representing a successful operation.
///
/// This function wraps the provided data in a `Response` with a success status and returns it
/// as an `Ok` variant of `RespResult`. It's a convenient way to generate a successful response
/// for operations that use the `RespResult` type.
///
/// # Type Parameters
/// - `'a`: A lifetime parameter, which is used to manage the lifetime of references within `BasicError`.
///   Although this function doesn't directly use the lifetime, it's part of the `RespResult` type signature.
/// - `T`: A generic type parameter representing the type of data to be wrapped in the response.
///
/// # Arguments
/// - `t`: The data to be included in the successful response.
///
/// # Returns
/// An `Ok` variant of `RespResult` containing a `Response` with the provided data.
/// The `Response` has a status code of `0` and a message of `"ok"`.
pub fn ok<'a, T>(t: T) -> RespResult<'a, T> {
    Ok(Response::ok(t))
}
