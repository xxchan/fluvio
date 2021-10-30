use regex::Regex;
use once_cell::sync::Lazy;
use fluvio_smartmodule::{smartmodule, Result, RecordData};
use fluvio_smartmodule::extract::*;

static SSN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{3}-\d{2}-\d{4}").unwrap());

#[smartmodule(map)]
pub fn map(record: Record<RecordData, String>) -> Result<(Option<RecordData>, RecordData)> {
    let output = SSN_RE
        .replace_all(record.value(), "***-**-****")
        .to_string();
    Ok((record.key, output.into()))
}
