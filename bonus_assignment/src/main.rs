use std::collections::HashMap;

use domain_info::{process, models};

fn main() -> Result<(), String> {
    let mut result: HashMap<&str, models::ResponseInfo> = HashMap::new();

    let url = "https://adrien-kiren-f22c-layer0-tech-test-default.layer0-limelight.link";
    let mut response =  ureq::get(url).call();

    //Ping until valid response
    while response.is_err() {
        response =  ureq::get(url).call();
    }
    let response = response.unwrap();
    let response_body = response.into_string().unwrap();

    let list_domains: Vec<&str> = serde_json::from_str(&response_body).unwrap();

    for domain in list_domains {
        result.insert(domain, process::process(domain)?);
    }

    println!("{}", serde_json::to_string_pretty(&result).unwrap());

    Ok(())
}