use fluvio_smartmodule::prelude::*;

#[smartmodule(aggregate)]
pub fn aggregate(acc: &str, current: Value<Parse<i32>>) -> Result<String> {
    // Parse the accumulator into an integer, or use default of 0
    let accumulator_int = acc.trim().parse::<i32>().unwrap_or(0);

    // Take the sum of the two integers and return it as a string
    let sum = accumulator_int + current.into_inner();
    Ok(sum.to_string())
}
