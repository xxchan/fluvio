use fluvio_smartmodule::{smartmodule, Result, RecordData};
use fluvio_smartmodule::extract::*;

/// This aggegrate concanate accumulator and current value
/// values: "a","b"
//  accumulator: "1",
//  "1a","1ab"
#[smartmodule(aggregate)]
pub fn aggregate(mut acc: String, current: Value<String>) -> Result<RecordData> {
    acc.push_str(current.inner());
    Ok(acc.into())
}
