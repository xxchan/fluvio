use std::collections::HashMap;
use fluvio_smartmodule::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
struct GithubStars(HashMap<String, u32>);

impl std::ops::Add for GithubStars {
    type Output = Self;

    fn add(mut self, next: Self) -> Self::Output {
        for (repo, new_stars) in next.0 {
            self.0
                .entry(repo)
                .and_modify(|stars| *stars += new_stars)
                .or_insert(new_stars);
        }
        self
    }
}

#[smartmodule(aggregate)]
fn aggregate(acc: &[u8], current: Value<Json<GithubStars>>) -> Result<Json<GithubStars>> {
    // Parse accumulator
    let accumulated_stars: GithubStars = serde_json::from_slice(acc).unwrap_or_default();

    // Add stars and serialize
    let summed_stars = accumulated_stars + current.into_inner();
    Ok(Json(summed_stars))
}
