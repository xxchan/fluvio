use fluvio_smartmodule::prelude::*;

#[smartmodule(map)]
pub fn map(record: Value<String>) -> Result<Value<String>> {
    let value = record.inner().to_ascii_lowercase();
    Ok(Value(value))
}
