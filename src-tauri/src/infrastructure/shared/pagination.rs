use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(default)]
pub struct Pagination {
    pub page: u64,
    pub size: u64,
}

impl Pagination {
    fn new(mut page: u64, mut size: u64) -> Self {
        if page == 0 {
            page = 1;
        }

        if size == 0 {
            size = 25;
        }

        Self { page, size }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, size: 25 }
    }
}

impl Pagination {
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.size
    }

    pub fn limit(&self) -> u64 {
        self.size
    }
}
