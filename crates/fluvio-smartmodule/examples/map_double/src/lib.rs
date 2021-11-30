use fluvio_smartmodule::{smartmodule, Result};
use fluvio_smartmodule::extract::*;

#[smartmodule(map)]
pub fn map(record: Value<Parse<i32>>) -> Result<Value<String>> {
    let value = (record.inner() * 2).to_string();
    Ok(Value(value))
}
