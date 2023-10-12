use std::env;

use serde_json::Value;

use crate::error::ReportGenerationError;
use crate::model::{AccountDetails, CallRecord, MessageRecord, UsageRecord};

fn make_request(endpoint: &str) -> Result<String, ReportGenerationError> {
    let twilio_account_sid =
        env::var("TWILIO_ACCOUNT_SID").expect("Twilio Account SID could not be retrieved.");
    let twilio_auth_token =
        env::var("TWILIO_AUTH_TOKEN").expect("Twilio Auth Token could not be retrieved.");

    let client = reqwest::blocking::Client::new();

    let request_url = format!("https://api.twilio.com/2010-04-01/Accounts/{twilio_account_sid}{endpoint}.json?PageSize=1000");
    let response = client
        .get(request_url)
        .basic_auth(twilio_account_sid, Some(twilio_auth_token))
        .send()?;
    let response_body = response.text()?;
    Ok(response_body)
}

pub fn get_account_details() -> Result<AccountDetails, ReportGenerationError> {
    let response = make_request("")?;
    let account_record: AccountDetails = serde_json::from_str(&response)?;
    Ok(account_record)
}

pub fn get_call_records() -> Result<Vec<CallRecord>, ReportGenerationError> {
    let response = make_request("/Calls")?;
    let mut response: Value = serde_json::from_str(&response)?;
    let call_records: Vec<CallRecord> = serde_json::from_value(response["calls"].take())?;
    Ok(call_records)
}

pub fn get_message_records() -> Result<Vec<MessageRecord>, ReportGenerationError> {
    let response = make_request("/Messages")?;
    let mut response: Value = serde_json::from_str(&response)?;
    let message_records: Vec<MessageRecord> = serde_json::from_value(response["messages"].take())?;
    Ok(message_records)
}

pub fn get_usage_records() -> Result<Vec<UsageRecord>, ReportGenerationError> {
    let response = make_request("/Usage/Records")?;
    let mut response: Value = serde_json::from_str(&response)?;
    let usage_records: Vec<UsageRecord> = serde_json::from_value(response["usage_records"].take())?;
    Ok(usage_records)
}