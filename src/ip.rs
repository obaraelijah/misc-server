use actix_web::{post, web, HttpResponse, Responder};
use std::borrow::Cow;

use crate::common::Config;

#[derive(serde::Deserialize)]
struct UpdateIpRequest {
    ip: String,
}

const OLD_IP_PATH: &str = "/tmp/old_ip.txt";

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
    let mut params: Vec<(String, String)> = vec![
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

    vec![("@", &server_ip), ("www", &server_ip), ("vpn", &new_ip)]
        .into_iter()
        .enumerate()
        .for_each(|(i, (sub_domain, ip))| {
            params.append(&mut record(i + 1, sub_domain.into(), ip.clone()));
        });

    params
}

#[post("update-ip")]
pub async fn update_ip(
    req: web::Json<UpdateIpRequest>,
    config: web::Data<Config>,
) -> impl Responder {
    let old_ip = std::fs::read_to_string(OLD_IP_PATH).unwrap_or("".to_string());
    if old_ip == req.ip {
        return HttpResponse::Ok().body("IP has not changed");
    }

    let params = create_request(
        Cow::from(&config.server_ip),
        Cow::from(&req.ip),
        Cow::from(&config.nc_api_key),
    );

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.namecheap.com/xml.response")
        .form(&params)
        .send()
        .await;

    return match res {
        Ok(_) => {
            std::fs::write(OLD_IP_PATH, &req.ip).unwrap();

            return HttpResponse::Ok().body(format!("Updated IP to {}", &req.ip));
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to update IP"),
    };
}
