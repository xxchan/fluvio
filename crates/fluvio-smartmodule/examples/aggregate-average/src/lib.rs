use serde::{Serialize, Deserialize};
use fluvio_smartmodule::prelude::*;

#[derive(Default, Serialize, Deserialize)]
struct IncrementalAverage {
    average: f64,
    count: u32,
}

impl IncrementalAverage {
    /// Implement the formula for calculating an incremental average.
    ///
    /// https://math.stackexchange.com/questions/106700/incremental-averageing
    fn add_value(&mut self, value: f64) {
        self.count += 1;
        let new_count_float = f64::from(self.count);
        let value_average_difference = value - self.average;
        let difference_over_count = value_average_difference / new_count_float;
        let new_average = self.average + difference_over_count;
        self.average = new_average;
    }
}

#[smartmodule(aggregate)]
fn aggregate(acc: &[u8], current: Value<Parse<f64>>) -> Result<Json<IncrementalAverage>> {
    // Parse the average from JSON
    let mut average: IncrementalAverage = serde_json::from_slice(acc).unwrap_or_default();
    average.add_value(current.into_inner());

    Ok(Json(average))
}
