//! This SmartModule takes JSON objects as inputs and returns key/value entries as output.
//!
//! JSON objects are made up of key/value entries, where the keys must be unique strings.
//! Consider the following JSON object:
//!
//! ```text
//! {
//!   "a": "Apple",
//!   "b": "Banana",
//!   "c": "Cranberry"
//! }
//! ```
//!
//! Another way to view this object is as an iterator over its key/value pairs:
//!
//! ```text
//! ...
//! ("a", "Apple")
//! ("b", "Banana")
//! ("c", "Cranberry")
//! ...
//! ```
//!
//! With this SmartModule, we use `#[smartmodule(array_map)]` to convert a stream
//! of JSON objects into a stream of all of the _children_ of those objects, using
//! the JSON object keys as the output record keys.
//!
//! To test this SmartModule, set up a test Topic:
//!
//! ```text
//! $ fluvio topic create array-map-object
//! ```
//!
//! Produce some valid JSON objects as input:
//!
//! ```text
//! $ fluvio produce array-map-object
//! > {"a": "Apple", "b": "Banana", "c": "Cranberry"}
//! Ok!
//! > ^C
//! ```
//!
//! Then, make sure you have compiled the SmartModule examples, and run the consumer:
//!
//! ```text
//! $ cd crates/fluvio-smartmodule/examples
//! $ cargo build --release
//! $ fluvio consume array-map-object -B --key-value --array-map=target/wasm32-unknown-unknown/release/fluvio_wasm_array_map_object.wasm
//! [a] "Apple"
//! [b] "Banana"
//! [c] "Cranberry"
//! ```

use fluvio_smartmodule::{smartmodule, Result, extract::*};
use serde_json::{Map, Value as SerdeValue};

#[smartmodule(array_map)]
pub fn array_map(
    record: Value<Json<Map<String, SerdeValue>>>,
) -> Result<Vec<Record<String, Json<SerdeValue>>>> {
    let output_records = record
        .into_inner() // Map<String, Value>
        .into_iter() // impl Iterator<Item = (String, Value)>
        // For every key-value pair in the JSON object, create a Record
        // whose key is the JSON key and whose value is that key's value
        .map(|(key, value): (String, SerdeValue)| Record {
            key: Some(key),
            value: Json(value),
        })
        .collect();

    Ok(output_records)
}
