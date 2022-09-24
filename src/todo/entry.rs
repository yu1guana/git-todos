// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

use anyhow::Result;
use chrono::{DateTime, Local};
use derive_new::new;
use getset::{Getters, Setters};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Setters, new)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub(crate) struct TodoEntry {
    title: String,
    description: String,
    updated_utc_datetime_rfc3339: String,
}

impl TodoEntry {
    pub(crate) fn updated_datetime(&self) -> Result<String> {
        let datetime = DateTime::parse_from_rfc3339(self.updated_utc_datetime_rfc3339())?
            .with_timezone(&Local);
        Ok(format!("{}", datetime.format("%Y/%m/%d %H:%M")))
    }
}
