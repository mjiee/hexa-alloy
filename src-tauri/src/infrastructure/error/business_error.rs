use crate::infrastructure::error::basic_error::BasicError;

pub const ERROR_FORGE_CLIENT_ERROR: BasicError<'static> = BasicError::BusinessError(10001, "forge client error");
pub const ERROR_STOCK_NOT_FOUND: BasicError<'static> = BasicError::BusinessError(20001, "stock not found");
