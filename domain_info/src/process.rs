use crate::models::ResponseInfo;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

use ureq::{Error, Response};
use dns_lookup::lookup_host;
use ssl_expiration::SslExpiration;

/// Main processing function for retrieving domain info
pub fn process(domain: &str) -> Result<ResponseInfo, String> {
    let info = get_http_info(domain)?;

    Ok(info)
}

/// Function to get all pertinent http information from a given domain
pub fn get_http_info(domain: &str) -> Result<ResponseInfo, String> {
    let url = &clean_url(domain);

    let result = Arc::new(Mutex::new(ResponseInfo::default()));

    match ureq::get(url).call() {
        Ok(response) => {
            let mut handles = vec![];

            let domain = domain.to_string();

            let response = Arc::new(response);
            let response_clone = Arc::clone(&response);

            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let http_status = get_status(&response_clone);
                result_clone.lock().unwrap().http_status = http_status;
            });
            handles.push(handle);

            let domain_clone = domain.clone();
            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let cert_valid = is_certificate_valid(&domain_clone);
                result_clone.lock().unwrap().cert_valid = cert_valid;
            });
            handles.push(handle);

            let response_clone = Arc::clone(&response);
            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let layer_0_version = layer0_version(&response_clone);
                result_clone.lock().unwrap().layer_0_version = layer_0_version;
            });
            handles.push(handle);

            let response_clone = Arc::clone(&response);
            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let layer_0_timings = layer0_timing(&response_clone);
                result_clone.lock().unwrap().layer_0_timings = layer_0_timings;
            });
            handles.push(handle);

            let domain_clone = domain.clone();
            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let ips = list_i_ps(&domain_clone);
                result_clone.lock().unwrap().ips = ips;
            });
            handles.push(handle);

            for handle in handles {
                handle.join().unwrap();
            }
            return Ok(result.lock().unwrap().clone())

        },
        Err(Error::Status(code, response)) => {
            let mut handles = vec![];

            let domain = domain.to_string();

            let response = Arc::new(response);

            let result_clone = Arc::clone(&result);
            result_clone.lock().unwrap().http_status = code;

            let domain_clone = domain.clone();
            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let cert_valid = is_certificate_valid(&domain_clone);
                result_clone.lock().unwrap().cert_valid = cert_valid;
            });
            handles.push(handle);

            let response_clone = Arc::clone(&response);
            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let layer_0_version = layer0_version(&response_clone);
                result_clone.lock().unwrap().layer_0_version = layer_0_version;
            });
            handles.push(handle);

            let response_clone = Arc::clone(&response);
            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let layer_0_timings = layer0_timing(&response_clone);
                result_clone.lock().unwrap().layer_0_timings = layer_0_timings;
            });
            handles.push(handle);

            let domain_clone = domain.clone();
            let result_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let ips = list_i_ps(&domain_clone);
                result_clone.lock().unwrap().ips = ips;
            });
            handles.push(handle);

            for handle in handles {
                handle.join().unwrap();
            }
            return Ok(result.lock().unwrap().clone())
        },
        Err(_) => {
           return Ok(ResponseInfo {
                http_status: 0,
                cert_valid: false,
                layer_0_version: None, 
                layer_0_timings: None,
                ips: vec![]
            })
        },
    }
}

/// Function to take domain and present clean url for processes
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

/// Function to retrieve http status from a response object
pub fn get_status(response: &Response) -> u16 {
    response.status()
}

/// Function to retrieve the cert valid status given a domain
pub fn is_certificate_valid(domain: &str) -> bool {
    let expiration = SslExpiration::from_domain_name(domain).unwrap();
    !expiration.is_expired()
}

/// Function to retrieve the layer0 version from the x-0-version response header
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

/// Function to retrieve the layer0 timing from the x-0-t response header
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

/// Function to retrieve all servicing IPs given a domain
pub fn list_i_ps(domain: &str) -> Vec<std::net::IpAddr> {
    let ips: Vec<std::net::IpAddr> = lookup_host(domain).unwrap();
    ips
}