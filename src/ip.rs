use std::{borrow::Cow, vec};

use crate::common::Config;

#[derive(serde::Deserialize)]
struct UpdateIpRequest {
    ip: String,
}

fn record(number: usize, sub_domain: Cow<str>, ip: Cow<str>) -> Vec<(String, String)> {
    return vec![
        (format!("HostName{}", number), sub_domain.to_string()),
        (format!("RecordType{}", number), "A".to_string()),
        (format!("Address{}", number), ip.to_string()),
        (format!("TTL{}", number), "1800".to_string()),
    ];
}

fn create_request(
    server_ip: Cow<str>,
    new_ip: Cow<str>,
    nc_api_key: Cow<str>,
) -> Vec<(String, String)> {
    let params  = vec![
        ("apiUser", "elijahobara"),
        ("apiKey", &nc_api_key),
        ("ClientIp", &server_ip),
        ("username", "elijahobara"),
        ("Command", "namecheap.domains.dns.setHosts"),
        ("SLD", "elijahtech"),
        ("TLD", "com"),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();

    return params;
}
