use std::io::Error;
use tracing::{debug, trace, instrument};

use dataplane::api::{RequestMessage, ResponseMessage, Request};
use dataplane::produce::DefaultProduceRequest;
use dataplane::fetch::DefaultFetchRequest;
use dataplane::versions::{ApiVersionKey, PlatformVersion};
use fluvio_spu_schema::server::SpuServerApiKey;
use fluvio_spu_schema::server::fetch_offset::FetchOffsetsRequest;
use fluvio_spu_schema::server::stream_fetch::DefaultStreamFetchRequest;
use fluvio_spu_schema::server::update_offset::UpdateOffsetsRequest;
use fluvio_spu_schema::{ApiVersionsRequest, ApiVersionsResponse};

#[instrument(skip(request))]
pub async fn handle_api_version_request(
    request: RequestMessage<ApiVersionsRequest>,
) -> Result<ResponseMessage<ApiVersionsResponse>, Error> {
    debug!("Handling ApiVersionsRequest");
    let mut response = ApiVersionsResponse::default();

    let platform_version = semver::Version::parse(&*crate::VERSION)
        .expect("Platform Version (from VERSION file) must be semver");
    response.platform_version = PlatformVersion::from(platform_version);

    response.api_keys.push(make_version_key(
        SpuServerApiKey::Produce,
        DefaultProduceRequest::MIN_API_VERSION,
        DefaultProduceRequest::MAX_API_VERSION,
    ));
    response.api_keys.push(make_version_key(
        SpuServerApiKey::Fetch,
        DefaultFetchRequest::MIN_API_VERSION,
        DefaultFetchRequest::MAX_API_VERSION,
    ));
    response.api_keys.push(make_version_key(
        SpuServerApiKey::FetchOffsets,
        FetchOffsetsRequest::DEFAULT_API_VERSION,
        FetchOffsetsRequest::DEFAULT_API_VERSION,
    ));
    response.api_keys.push(make_version_key(
        SpuServerApiKey::StreamFetch,
        0,
        DefaultStreamFetchRequest::DEFAULT_API_VERSION,
    ));
    response.api_keys.push(make_version_key(
        SpuServerApiKey::UpdateOffsets,
        0,
        UpdateOffsetsRequest::DEFAULT_API_VERSION,
    ));

    trace!("Returning ApiVersionsResponse: {:#?}", &response);
    Ok(request.new_response(response))
}

/// Build version key object
fn make_version_key(key: SpuServerApiKey, min_version: i16, max_version: i16) -> ApiVersionKey {
    let api_key = key as i16;
    ApiVersionKey {
        api_key,
        min_version,
        max_version,
    }
}
