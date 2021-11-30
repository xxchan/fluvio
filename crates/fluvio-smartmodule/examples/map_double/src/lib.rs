use fluvio_smartmodule::prelude::*;

#[smartmodule(map)]
pub fn map(record: Value<Parse<i32>>) -> Result<Value<String>> {
    let value = (record.inner() * 2).to_string();
    Ok(Value(value))
}
