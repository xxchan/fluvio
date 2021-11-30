//! An ArrayMap for breaking apart a paginated Reddit API response into individual posts.
//!
//! The SmartModules ArrayMap function allows you map a single input Record into
//! zero or many output records. This example showcases taking a stream of Reddit API
//! responses and converting it into a stream of the individual posts.

use fluvio_smartmodule::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct RedditListing {
    data: RedditPage,
}

#[derive(Debug, Serialize, Deserialize)]
struct RedditPage {
    children: Vec<RedditPost>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RedditPost {
    data: RedditPostData,
}

#[derive(Debug, Serialize, Deserialize)]
struct RedditPostData {
    id: String,
    title: String,
    url: String,
    selftext: String,
    ups: i32,
    upvote_ratio: f32,
}

#[smartmodule(array_map)]
fn array_map(
    record: Value<Json<RedditListing>>,
) -> Result<Vec<Record<String, Json<RedditPostData>>>> {
    let records = record
        .into_inner()
        .data
        .children
        .into_iter()
        .map(|post: RedditPost| Record {
            key: Some(post.data.id.clone()),
            value: Json(post.data),
        })
        .collect();

    Ok(records)
}
