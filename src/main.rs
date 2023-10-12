use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use dotenvy::dotenv;

use crate::error::ReportGenerationError;
use crate::model::AccountDetails;
use crate::writer::write_results;

mod model;
mod twilio;
mod writer;
mod error;

fn main() -> Result<(), ReportGenerationError> {
    dotenv().ok();
    println!("Starting synchronous execution");
    let start = Instant::now();
    generate_report_synchronously()?;
    println!("Synchronous Execution Time: {:?}", start.elapsed());

    println!("Starting concurrent execution");
    let start = Instant::now();
    generate_report_concurrently()?;
    println!("Concurrent Execution Time: {:?}", start.elapsed());

    Ok(())
}

pub fn generate_report_synchronously() -> Result<(), ReportGenerationError> {
    let account_details = twilio::get_account_details()?;
    let usage_records = twilio::get_usage_records()?;
    let message_records = twilio::get_message_records()?;
    let call_records = twilio::get_call_records()?;

    write_results(account_details, usage_records, message_records, call_records)?;

    Ok(())
}

pub fn generate_report_concurrently() -> Result<(), ReportGenerationError> {
    let account_record = Arc::new(Mutex::new(AccountDetails::empty()));
    let usage_records = Arc::new(Mutex::new(vec![]));
    let message_records = Arc::new(Mutex::new(vec![]));
    let call_records = Arc::new(Mutex::new(vec![]));

    let mut handles = vec![];

    let cloned_record = account_record.clone();
    handles.push(thread::spawn(move || {
        let mut record = cloned_record.lock().unwrap();
        *record = twilio::get_account_details().unwrap();
    }));

    let cloned_usage = usage_records.clone();
    handles.push(thread::spawn(move || {
        let mut result = cloned_usage.lock().unwrap();
        *result = twilio::get_usage_records().unwrap();
    }));

    let cloned_messages = message_records.clone();
    handles.push(thread::spawn(move || {
        let mut result = cloned_messages.lock().unwrap();
        *result = twilio::get_message_records().unwrap();
    }));

    let cloned_calls = call_records.clone();
    handles.push(thread::spawn(move || {
        let mut result = cloned_calls.lock().unwrap();
        *result = twilio::get_call_records().unwrap();
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    let account_details = account_record.lock().unwrap().to_owned();
    let usage_records = usage_records.lock().unwrap().to_vec();
    let message_records = message_records.lock().unwrap().to_vec();
    let call_records = call_records.lock().unwrap().to_vec();

    write_results(
        account_details, usage_records.to_vec(), message_records, call_records,
    )?;

    Ok(())
}
