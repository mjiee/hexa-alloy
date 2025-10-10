use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult<T> {
    pub data: Option<Vec<T>>,
    pub total: u64,
}

impl<T> QueryResult<T> {
    pub fn new(data: Option<Vec<T>>, total: u64) -> Self {
        QueryResult { data, total }
    }
}
