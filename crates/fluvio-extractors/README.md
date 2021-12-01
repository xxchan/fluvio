# `fluvio-extractors`

Fluvio extractors are tools that allow you to easily work with raw data
in a structured format. They can be used in SmartModules to read Record
data to an in-memory format that is easier to manipulate.

## Simple usage in a SmartModule

Let's say we have a SmartModule whose input records contain JSON that
looks like the following:

```
{"star_rating":5,"review":"This is my favorite restaurant!"}
{"star_rating":4,"review":"Great food, worth the wait"}
{"star_rating":3,"review":"Good for a quick eat, but not date night"}
```

If we wanted to write a SmartModule to extract just the `star_rating`
values, we could use Fluvio extractors to pull out the JSON values we want, like so:

```rust
use fluvio_smartmodule::prelude::*;
use serde_json::Value as SerdeValue;

#[smartmodule(map)]
fn map(record: Value<Json<SerdeValue>>) -> Result<Value<Json<SerdeValue>>> {
    let value: SerdeValue = record.into_inner();
    let star_rating = value["star_rating"].to_owned();
    Ok(Value(Json(star_rating)))
}
```

Here, we are using the `Value` and `Json` extractors. The `Value` extractor says that
we want to read the Value of the Record (as opposed to the Key), and the `Json` extractor
says that we want to interpret the Record's data as a JSON value.

Note that the `Json` extractor is actually generic, `Json<T>`. You may use any type
that implements serde's `Deserialize` and `Serialize` for `T`. In this example, we might want
to write a custom data type instead of using `serde_json::Value`. When we do that, our
SmartModule would look like this:

```rust
use serde::{Serialize, Deserialize};
use fluvio_smartmodule::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Review {
    star_rating: i32,
    review: String,
}

#[smartmodule(map)]
fn map(record: Value<Json<Review>>) -> Result<Value<Json<i32>>> {
    let review: Review = record.into_inner();
    Ok(Value(Json(review.star_rating)))
}
```

Here, notice that we're using `Value<Json<Review>>` as our input type. This lets us extract
the `Review` to use directly when we call `record.into_inner()`. For our return type, we're
using `Value<Json<i32>>`, so our output records will just be JSON integers rather than a JSON _object_
(remember, integers can be top-level JSON values). Running this SmartModule will take our input
from above and give an output stream with the following contents, where each line is one record:

```
5
4
3
```

## Extractor types

Extractors are all about stacking types to define how to get your data from a raw binary format
to a workable in-memory format. There are two kinds of extractors that work together: "record extractors"
and "data extractors". The record extractors are the following:

- `Value<V>`: Extracts from the Value data of a record
- `Key<K>`: Extracts from the Key data of a record
- `Record<K, V>`: Extracts both the Key and Value data of a record

We've already seen the `Value` extractor above, when we used it to extract our record's value data as JSON.
`Value` is the most commonly used record-extractor, use it when you don't need to read or transform the
Key of a record. If you _only_ want to edit the Key of a record, use `Key<K>`, if you want to read both
the key and value, use `Record<K, V>`.

So "record extractors" let us say what part of the record data we want to read (key, value, or both),
and "data extractors" let us say _how_ we want to interpret that data as an in-memory type.
The currently available data extractors are:

- `Json<T>`: Extract JSON data as an in-memory type `T`, as long as `T: serde::Deserialize`
- `Yaml<T>`: Extract YAML data as an in-memory type `T`, as long as `T: serde::Deserialize`
- `Toml<T>`: Extract TOML data as an in-memory type `T`, as long as `T: serde::Deserialize`
- `&[u8]`: Expose the binary data as a byte slice
- `Vec<u8>`: Expose the binary data as a vector
- `Bytes`: Expose the binary data as a `bytes::Bytes` struct
- `&str`: Extract the data as a UTF-8 encoded string slice
- `String`: Extract the data as a UTF-8 encoded string
- `Parse<T>`: Extract the UTF-8 encoded data as an in-memory type `T`, as long as `T: FromStr`

Most extractors may also be used as return values from SmartModules, in which case they will take
their in-memory values and write them back to a binary format. For most types, this behaves exactly
as expected: `&[u8]`, `Vec<u8>`, `Bytes`, `&str`, and `String` write their byte contents directly, and
`Json<T>`, `Yaml<T>`, and `Toml<T>` will serialize their inner `T` as json, yaml, or toml, respectively
(as long as `T: serde::Serialize`). One exception to the reversal rule is `Parse<T>`, which cannot be
used in return position.
