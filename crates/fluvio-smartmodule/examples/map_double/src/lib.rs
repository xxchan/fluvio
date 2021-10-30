use fluvio_smartmodule::{smartmodule, Result, RecordData};
use fluvio_smartmodule::extract::*;

#[smartmodule(map)]
pub fn map(record: Record<RecordData, Parse<i32>>) -> Result<(Option<RecordData>, RecordData)> {
    let value = (record.value() * 2).to_string();
    Ok((record.key, value.into()))
}
