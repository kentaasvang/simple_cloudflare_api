// METHODS
// get zones
// get records in zones
// update records

// TYPES
// Zone
// DNSRecord

use serde::Deserialize;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

pub struct DNSRecord {}

#[derive(Deserialize, Debug)]
struct CloudflareResponse<T> {
    success: bool,
    messages: Vec<String>,
    errors: Vec<String>,
    // result_info:
    result: Vec<T>,
}


#[derive(Deserialize, Debug)]
pub struct Zone {
    id: String,
    name: String,
}


pub async fn list_zones(token: String) -> Vec<Zone> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    let url = "https://api.cloudflare.com/client/v4/zones";

    let mut bearer_token = String::from("Bearer");
    bearer_token.push_str(" ");
    bearer_token.push_str(token.as_str());
    let auth_value = HeaderValue::from_str(bearer_token.as_str());

    headers.insert(AUTHORIZATION, auth_value.unwrap());

    let resp = client.get(url)
        .headers(headers)
        .send()
        .await
        .expect("Request failed")
        .text()
        .await
        .expect("decoding failed");

    let val: CloudflareResponse<Zone> = serde_json::from_str(&resp).unwrap();
    val.result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
