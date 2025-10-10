use std::io;

use serde::{ser::SerializeStruct as _, Serialize, Serializer};

/// Basic error
#[derive(Debug)]
pub enum BasicError<'a> {
    /// Internal error
    InternalError(anyhow::Error),

    /// Business error
    BusinessError(i32, &'a str),
}

impl<'a> Serialize for BasicError<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Response", 2)?;

        match self {
            BasicError::InternalError(err) => {
                tracing::error!(error = %err);
                state.serialize_field("code", &500)?;
                state.serialize_field("message", &"Internal Error")?;
            }
            BasicError::BusinessError(code, message) => {
                state.serialize_field("code", code)?;
                state.serialize_field("message", &message.to_string())?;
            }
        }

        state.end()
    }
}

impl std::fmt::Display for BasicError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError(err) => write!(f, "{}", err),
            Self::BusinessError(code, message) => write!(f, "code: {}, message: {}", code, message),
        }
    }
}

impl std::error::Error for BasicError<'_> {}

impl From<io::Error> for BasicError<'_> {
    fn from(err: io::Error) -> Self {
        Self::InternalError(anyhow::Error::new(err))
    }
}

impl From<reqwest::Error> for BasicError<'_> {
    fn from(err: reqwest::Error) -> Self {
        Self::InternalError(anyhow::Error::new(err))
    }
}

impl From<url::ParseError> for BasicError<'_> {
    fn from(value: url::ParseError) -> Self {
        Self::InternalError(anyhow::Error::new(value))
    }
}

impl From<serde_json::Error> for BasicError<'_> {
    fn from(value: serde_json::Error) -> Self {
        Self::InternalError(anyhow::Error::new(value))
    }
}

impl From<sea_orm::DbErr> for BasicError<'_> {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::InternalError(anyhow::Error::new(value))
    }
}
