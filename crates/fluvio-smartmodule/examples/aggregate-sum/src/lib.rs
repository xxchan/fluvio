use fluvio_smartmodule::{smartmodule, Result, RecordData, extract::*};

#[smartmodule(aggregate)]
pub fn aggregate(acc: &[u8], current: Value<Parse<i32>>) -> Result<RecordData> {
    // Parse the accumulator as a string
    let accumulator_string = std::str::from_utf8(acc)?;

    // Parse the string into an integer
    let accumulator_int = accumulator_string.trim().parse::<i32>().unwrap_or(0);

    // Take the sum of the two integers and return it as a string
    let sum = accumulator_int + current.into_inner();
    Ok(sum.to_string().into())
}
