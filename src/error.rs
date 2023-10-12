use std::error::Error;
use std::fmt::{Display, Formatter};
use rust_xlsxwriter::XlsxError;

#[derive(Debug)]
pub enum ReportGenerationError {
    RetrieveRecordError(reqwest::Error),
    ParseRecordError(serde_json::Error),
    WriteReportError(XlsxError),
}

impl From<XlsxError> for ReportGenerationError {
    fn from(e: XlsxError) -> Self {
        Self::WriteReportError(e)
    }
}

impl From<reqwest::Error> for ReportGenerationError {
    fn from(e: reqwest::Error) -> Self {
        Self::RetrieveRecordError(e)
    }
}

impl From<serde_json::Error> for ReportGenerationError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseRecordError(e)
    }
}

impl Error for ReportGenerationError {}

impl Display for ReportGenerationError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ReportGenerationError::RetrieveRecordError(e) => { write!(f, "error retrieving results {}", e) }
            ReportGenerationError::ParseRecordError(e) => { write!(f, "error parsing results {}", e) }
            ReportGenerationError::WriteReportError(e) => { write!(f, "error generating report {}", e) }
        }
    }
}
