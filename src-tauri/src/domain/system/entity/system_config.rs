use serde::de::DeserializeOwned;

use crate::infrastructure::error::BasicResult;

pub struct SystemConfig {
    pub key: String,
    pub value: serde_json::Value,
}

impl SystemConfig {
    pub fn into_value<'a, T>(self) -> BasicResult<'a, T>
    where
        T: DeserializeOwned,
    {
        let value = serde_json::from_value(self.value)?;
        Ok(value)
    }
}
