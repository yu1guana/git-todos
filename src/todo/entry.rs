// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

use getset::{Getters, Setters};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub(crate) struct TodoEntry {
    title: String,
    description: String,
    updated_utc_datetime_rfc3339: String,
}

impl TodoEntry {
    pub(crate) fn new(
        title: String,
        description: String,
        updated_utc_datetime_rfc3339: String,
    ) -> Self {
        Self {
            title,
            description,
            updated_utc_datetime_rfc3339,
        }
    }
}
