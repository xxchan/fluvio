use regex::Regex;
use once_cell::sync::Lazy;
use fluvio_smartmodule::prelude::*;

static SSN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{3}-\d{2}-\d{4}").unwrap());

#[smartmodule(map)]
pub fn map(record: Value<String>) -> Result<Value<String>> {
    let output = SSN_RE
        .replace_all(record.inner(), "***-**-****")
        .to_string();
    Ok(Value(output))
}
