
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AccountDetails {
    pub friendly_name: String,
    pub sid: String,
    pub date_created: String,
    pub status: String,
    #[serde(rename = "type")]
    pub account_type: String,
}

impl AccountDetails {
    pub fn empty() -> Self {
        AccountDetails {
            friendly_name: String::from(""),
            sid: String::from(""),
            date_created: String::from(""),
            status: String::from(""),
            account_type: String::from(""),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UsageRecord {
    pub account_sid: String,
    pub category: String,
    pub description: String,
    pub usage_unit: Option<String>,
    pub usage: String,
    pub price: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageRecord {
    pub sid: String,
    pub account_sid: String,
    pub date_created: String,
    pub date_sent: String,
    pub from: String,
    pub to: String,
    pub status: String,
    pub num_segments: String,
    pub num_media: String,
    pub price: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CallRecord {
    pub sid: String,
    pub account_sid: String,
    pub date_created: String,
    pub from: String,
    pub to: String,
    pub status: String,
    pub start_time: String,
    pub end_time: String,
    pub price: Option<String>,
}
