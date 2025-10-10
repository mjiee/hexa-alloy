pub mod basic_error;
pub mod business_error;

pub use basic_error::*;
pub use business_error::*;

pub type BasicResult<'a, T> = Result<T, BasicError<'a>>;
