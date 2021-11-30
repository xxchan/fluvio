//! This SmartModule takes JSON arrays as inputs and returns the values in those arrays as output.
//!
//! Sometimes we want to take a composite value like a JSON array and break it down into
//! it's component pieces, i.e. the values inside the array. We can do this using a
//! SmartModule Flatmap.
//!
//! In this example, we'll take a stream of JSON arrays as input, and we'll return a stream
//! of all the child _values_ that were in those arrays as output.
//!
//! To test this SmartModule, set up a test Topic:
//!
//! ```text
//! $ fluvio topic create array-map-array
//! ```
//!
//! Produce some valid JSON arrays as input:
//!
//! ```text
//! $ fluvio produce array-map-array
//! > ["Apple", "Banana", Cranberry"]
//! Ok!
//! > ^C
//! ```
//!
//! Then, make sure you have compiled the SmartModule examples, and run the consumer:
//!
//! ```text
//! $ cd crates/fluvio-smartmodule/examples
//! $ cargo build --release
//! $ fluvio consume array-map-array -B --array-map=target/wasm32-unknown-unknown/release/fluvio_wasm_array_map_array.wasm
//! "Apple"
//! "Banana"
//! "Cranberry"
//! ```

use serde_json::Value as SerdeValue;
use fluvio_smartmodule::{smartmodule, Result, extract::*};

#[smartmodule(array_map)]
pub fn array_map(record: Value<Json<Vec<SerdeValue>>>) -> Result<Vec<Value<Json<SerdeValue>>>> {
    let values = record
        .into_inner()
        .into_iter()
        .map(|v| Value(Json(v)))
        .collect();
    Ok(values)
}
