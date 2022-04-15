use crate::models;

use std::collections::HashMap;

use ureq::{Error, Response};
use dns_lookup::lookup_host;
use ssl_expiration::SslExpiration;

pub fn process(domain: &str) -> Result<models::ResponseInfo, String> {
    // Get pertinent info
    let info = get_http_info(domain)?;

    Ok(info)
}

pub fn clean_url(url: &str) -> String {
    match url.starts_with("http://") || url.starts_with("http://") {
        true => {
            return String::from(url)
        },
        false => {
            return format!("http://{}", url)
        }
    }
}

pub fn get_http_info(domain: &str) -> Result<models::ResponseInfo, String> {
    let url = &clean_url(domain);

    match ureq::get(url).call() {
        Ok(response) => {
            Ok(models::ResponseInfo {
                http_status: get_status(&response),
                cert_valid: is_certificate_valid(domain),
                layer_0_version: layer0_version(&response), 
                layer_0_timings: layer0_timing(&response),
                ips: list_i_ps(domain)
            })
        },
        Err(Error::Status(code, response)) => {
            Ok(models::ResponseInfo {
                http_status: code,
                cert_valid: is_certificate_valid(domain),
                layer_0_version: layer0_version(&response), 
                layer_0_timings: layer0_timing(&response),
                ips: list_i_ps(domain)
            })
        },
        Err(_) => {
            Ok(models::ResponseInfo {
                http_status: 0,
                cert_valid: false,
                layer_0_version: None, 
                layer_0_timings: None,
                ips: vec![]
            })
        },
    }
}

pub fn get_status(response: &Response) -> u16 {
    response.status()
}

pub fn is_certificate_valid(domain: &str) -> bool {
    let expiration = SslExpiration::from_domain_name(domain).unwrap();
    !expiration.is_expired()
}

pub fn layer0_version(response: &Response) -> Option<String> {
    match response.header("x-0-version") {
        Some(header) => {
            let split_header: Vec<&str> = header.split(" ").collect();
            return Some(split_header[1].to_string())
        },
        None => {
            return None
        },
    }
}

pub fn layer0_timing(response: &Response) -> Option<HashMap<String, u16>> {
    match response.header("x-0-t") {
        Some(header) => {
            let mut layer0_timing: HashMap<String, u16> = HashMap::new();

            let split_header: Vec<&str> = header.split(",").collect();
            for instance in split_header {
                let split_instance: Vec<&str> = instance.split("=").collect();

                if split_instance[0].ends_with("t") {
                    let name = String::from(split_instance[0]);
                    let val = String::from(split_instance[1]).parse::<u16>().unwrap();

                    layer0_timing.insert(name, val);
                }
            }
            return Some(layer0_timing)
        },
        None => {
            return None
        },
    }
}

pub fn list_i_ps(domain: &str) -> Vec<std::net::IpAddr> {
    let ips: Vec<std::net::IpAddr> = lookup_host(domain).unwrap();
    ips
}