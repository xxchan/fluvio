use fluvio_smartmodule::{smartmodule, Result, RecordData};
use fluvio_smartmodule::extractors::{Record, bytes::Slice, string::Parse};

#[smartmodule(map)]
pub fn map(record: Record<Slice, Parse<i32>>) -> Result<(Option<RecordData>, RecordData)> {
    let value = (record.value.0 * 2).to_string();
    Ok((record.key.map(|k| k.into()), value.into()))
}
