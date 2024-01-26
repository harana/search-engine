use curl::easy::Easy;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use url::Url;
use sha256::digest;
use base64::{Engine as _, engine::general_purpose};
use crate::urlencoder::encode;
use std::collections::BTreeMap;

pub fn purge_url(purge_url: &str, base_url: &str, security_key: &str) {
    let mut http_client = Easy::new();

    // Purge URL from Bunny
    let url = format!("{}/purge?url={}", base_url, purge_url);
    http_client.url(url.as_str()).unwrap();

    // Refetch the URL
    let signed_url = sign_url(&url, &security_key, None, None, false, None, None, None);
    http_client.url(&signed_url).unwrap();
}

// Based off the official Bunny code: https://github.com/BunnyWay/BunnyCDN.TokenAuthentication
pub fn sign_url(original_url: &str,
                security_key: &str,
                expiration_time: Option<u64>,
                user_ip: Option<&str>,
                is_directory: bool,
                path_allowed: Option<&str>,
                countries_allowed: Option<Vec<&str>>,
                countries_blocked: Option<Vec<&str>>) -> String {

    let expiration = Duration::from_secs(expiration_time.unwrap_or(3600));
    let expires = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().checked_add(expiration);

    let mut url = Url::parse(original_url).unwrap();
    if countries_allowed.is_some() {
        url.query_pairs_mut().append_pair("token_countries", &*countries_allowed.unwrap().join(","));
    }
    if countries_blocked.is_some() {
        url.query_pairs_mut().append_pair("token_countries_blocked", &*countries_blocked.unwrap().join(","));
    }

    let url_query = url.query_pairs().into_owned();
    let mut parameters = BTreeMap::from_iter(url_query);
    let url_path = &url.path().to_owned();
    let signature_path = if path_allowed.is_some() { path_allowed.unwrap() } else { url_path };
    if path_allowed.is_some() { parameters.insert("token_path".to_string(), signature_path.to_string()); };
    url.query_pairs_mut().clear().extend_pairs(parameters.to_owned());

    let mut encoded_parameters_str = String::new();
    for (key, value) in &parameters {
        encoded_parameters_str.push_str(format!("&{}={}", key, encode(value)).as_str());
    }

    let digest = digest(format!("{}{}{}{}{}", security_key, signature_path, expires.unwrap().as_secs(), url.query().unwrap(), user_ip.unwrap_or("")));
    let base64 = general_purpose::STANDARD_NO_PAD.encode(digest);
    let token = base64.replace("\n", "").replace("+", "-").replace("/", "_").replace("=", "");

    if is_directory {
        format!("{}://{}/bcdn_token={}{}&expires={}{}", url.scheme(), url.host().unwrap(), token, encoded_parameters_str, expires.unwrap().as_secs(), url.path())
    }else{
        format!("{}://{}{}?token={}{}&expires={}", url.scheme(), url.host().unwrap(), url.path(), token, encoded_parameters_str, expires.unwrap().as_secs())
    }
}