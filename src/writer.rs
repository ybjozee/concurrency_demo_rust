use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, RowNum, Workbook};

use crate::error::ReportGenerationError;
use crate::model::{AccountDetails, CallRecord, MessageRecord, UsageRecord};

pub fn write_results(account_details: AccountDetails, usage_records: Vec<UsageRecord>, message_records: Vec<MessageRecord>, call_records: Vec<CallRecord>) -> Result<(), ReportGenerationError> {
    let mut workbook = Workbook::new();

    write_account_details(&mut workbook, account_details)?;
    write_usage_records(&mut workbook, usage_records)?;
    write_message_records(&mut workbook, message_records)?;
    write_call_records(&mut workbook, call_records)?;

    workbook.save("Usage report.xlsx")?;

    Ok(())
}

fn write_account_details(workbook: &mut Workbook, account_details: AccountDetails) -> Result<(), ReportGenerationError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Account")?;

    let header_format = get_header_format();

    worksheet.write_with_format(0, 0, "Friendly name", &header_format)?;
    worksheet.write_with_format(0, 1, "SID", &header_format)?;
    worksheet.write_with_format(0, 2, "Date created", &header_format)?;
    worksheet.write_with_format(0, 3, "Status", &header_format)?;
    worksheet.write_with_format(0, 4, "Type", &header_format)?;

    let row_format = get_row_format();

    worksheet.write_with_format(1, 0, account_details.friendly_name, &row_format)?;
    worksheet.write_with_format(1, 1, account_details.sid, &row_format)?;
    worksheet.write_with_format(1, 2, account_details.date_created, &row_format)?;
    worksheet.write_with_format(1, 3, account_details.status, &row_format)?;
    worksheet.write_with_format(1, 4, account_details.account_type, &row_format)?;

    worksheet.autofit();

    Ok(())
}

fn write_usage_records(workbook: &mut Workbook, usage_records: Vec<UsageRecord>) -> Result<(), ReportGenerationError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("All time usage")?;

    let header_format = get_header_format();

    worksheet.write_with_format(0, 0, "S/N", &header_format)?;
    worksheet.write_with_format(0, 1, "Account SID", &header_format)?;
    worksheet.write_with_format(0, 2, "Category", &header_format)?;
    worksheet.write_with_format(0, 3, "Description", &header_format)?;
    worksheet.write_with_format(0, 4, "Usage unit", &header_format)?;
    worksheet.write_with_format(0, 5, "Usage", &header_format)?;
    worksheet.write_with_format(0, 6, "Price", &header_format)?;

    let row_format = get_row_format();

    for (i, record) in usage_records.iter().enumerate() {
        let row_index: RowNum = (i + 1) as RowNum;
        worksheet.write_with_format(row_index, 0, row_index, &row_format)?;
        worksheet.write_with_format(row_index, 1, &record.account_sid, &row_format)?;
        worksheet.write_with_format(row_index, 2, &record.category, &row_format)?;
        worksheet.write_with_format(row_index, 3, &record.description, &row_format)?;
        if let Some(usage_unit) = &record.usage_unit {
            worksheet.write_with_format(row_index, 4, usage_unit, &row_format)?;
        } else {
            worksheet.write_with_format(row_index, 4, "", &row_format)?;
        }
        worksheet.write_with_format(row_index, 5, &record.usage, &row_format)?;
        worksheet.write_with_format(row_index, 6, &record.price, &row_format)?;
    }
    worksheet.autofit();
    Ok(())
}

fn write_message_records(workbook: &mut Workbook, message_records: Vec<MessageRecord>) -> Result<(), ReportGenerationError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Messages")?;

    let header_format = get_header_format();

    worksheet.write_with_format(0, 0, "S/N", &header_format)?;
    worksheet.write_with_format(0, 1, "SID", &header_format)?;
    worksheet.write_with_format(0, 2, "Account SID", &header_format)?;
    worksheet.write_with_format(0, 3, "Date created", &header_format)?;
    worksheet.write_with_format(0, 4, "Date sent", &header_format)?;
    worksheet.write_with_format(0, 5, "From", &header_format)?;
    worksheet.write_with_format(0, 6, "To", &header_format)?;
    worksheet.write_with_format(0, 7, "Status", &header_format)?;
    worksheet.write_with_format(0, 8, "Segments", &header_format)?;
    worksheet.write_with_format(0, 9, "Media", &header_format)?;
    worksheet.write_with_format(0, 10, "Price", &header_format)?;

    let row_format = get_row_format();
    for (i, record) in message_records.iter().enumerate() {
        let row_index: RowNum = (i + 1) as RowNum;
        worksheet.write_with_format(row_index, 0, row_index, &row_format)?;
        worksheet.write_with_format(row_index, 1, &record.sid, &row_format)?;
        worksheet.write_with_format(row_index, 2, &record.account_sid, &row_format)?;
        worksheet.write_with_format(row_index, 3, &record.date_created, &row_format)?;
        worksheet.write_with_format(row_index, 4, &record.date_sent, &row_format)?;
        worksheet.write_with_format(row_index, 5, &record.from, &row_format)?;
        worksheet.write_with_format(row_index, 6, &record.to, &row_format)?;
        worksheet.write_with_format(row_index, 7, &record.status, &row_format)?;
        worksheet.write_with_format(row_index, 8, &record.num_segments, &row_format)?;
        worksheet.write_with_format(row_index, 9, &record.num_media, &row_format)?;
        if let Some(price) = &record.price {
            worksheet.write_with_format(row_index, 10, price, &row_format)?;
        } else {
            worksheet.write_with_format(row_index, 10, "", &row_format)?;
        }
    }

    worksheet.autofit();
    Ok(())
}

fn write_call_records(workbook: &mut Workbook, call_records: Vec<CallRecord>) -> Result<(), ReportGenerationError> {
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Calls")?;

    let header_format = get_header_format();

    worksheet.write_with_format(0, 0, "S/N", &header_format)?;
    worksheet.write_with_format(0, 1, "SID", &header_format)?;
    worksheet.write_with_format(0, 2, "Account SID", &header_format)?;
    worksheet.write_with_format(0, 3, "Date created", &header_format)?;
    worksheet.write_with_format(0, 4, "From", &header_format)?;
    worksheet.write_with_format(0, 5, "To", &header_format)?;
    worksheet.write_with_format(0, 6, "Status", &header_format)?;
    worksheet.write_with_format(0, 7, "Start time", &header_format)?;
    worksheet.write_with_format(0, 8, "End time", &header_format)?;
    worksheet.write_with_format(0, 9, "Price", &header_format)?;

    let row_format = get_row_format();

    for (i, record) in call_records.iter().enumerate() {
        let row_index: RowNum = (i + 1) as RowNum;
        worksheet.write_with_format(row_index, 0, row_index, &row_format)?;
        worksheet.write_with_format(row_index, 1, &record.sid, &row_format)?;
        worksheet.write_with_format(row_index, 2, &record.account_sid, &row_format)?;
        worksheet.write_with_format(row_index, 3, &record.date_created, &row_format)?;
        worksheet.write_with_format(row_index, 4, &record.from, &row_format)?;
        worksheet.write_with_format(row_index, 5, &record.to, &row_format)?;
        worksheet.write_with_format(row_index, 6, &record.status, &row_format)?;
        worksheet.write_with_format(row_index, 7, &record.start_time, &row_format)?;
        worksheet.write_with_format(row_index, 8, &record.end_time, &row_format)?;
        if let Some(price) = &record.price {
            worksheet.write_with_format(row_index, 9, price, &row_format)?;
        } else {
            worksheet.write_with_format(row_index, 9, "", &row_format)?;
        }
    }

    worksheet.autofit();
    Ok(())
}

fn get_header_format() -> Format {
    Format::new()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thick)
        .set_bold()
}

fn get_row_format() -> Format {
    Format::new()
        .set_align(FormatAlign::Justify)
        .set_border(FormatBorder::Thin)
}