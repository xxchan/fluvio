use fluvio_smartmodule::{smartmodule, Result};
use fluvio_smartmodule::extract::*;
use regex::Regex;

#[smartmodule(filter)]
pub fn filter(value: Value<String>) -> Result<bool> {
    // Check whether the Record contains a Social Security number
    let social_security_regex = Regex::new(r"\d{3}-\d{2}-\d{4}").unwrap();
    let has_ss = social_security_regex.is_match(value.inner());

    // Only accept records that _do not_ have social security numbers in them
    Ok(!has_ss)
}
