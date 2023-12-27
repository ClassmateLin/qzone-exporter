use reqwest::Client;

use super::error::QZoneError;

const UA: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36";

pub async fn get_http_client() -> Result<Client, QZoneError> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .https_only(true)
        .user_agent(UA)
        .build()
        .map_err(|_| QZoneError::ReqwestError)?;

    Ok(client)
}

pub async fn get_http2_client() -> Result<Client, QZoneError> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .http2_prior_knowledge()
        .use_rustls_tls()
        .user_agent(UA)
        .https_only(true)
        .build()
        .map_err(|_| QZoneError::ReqwestError)?;

    Ok(client)
}
