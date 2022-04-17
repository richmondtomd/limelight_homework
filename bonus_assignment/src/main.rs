use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

use domain_info::{process, models};

fn main() -> Result<(), String> {
    // let mut result: HashMap<&str, models::ResponseInfo> = HashMap::new();
    let result = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    let url = "https://adrien-kiren-f22c-layer0-tech-test-default.layer0-limelight.link";
    let mut response =  ureq::get(url).call();

    //Ping until valid response
    while response.is_err() {
        response =  ureq::get(url).call();
    }

    //Get Response body
    let response = response.unwrap();
    let response_body = response.into_string().unwrap();

    //Retrieve list of domains from response body
    let list_domains: Vec<String> = serde_json::from_str(&response_body).unwrap();

    //Loop through domains
    for domain in list_domains {
        let inner_result = Arc::clone(&result);
        //Spawn thread to process domain
        let handle = thread::spawn(move || {
            let domain_info = process::process(domain.as_ref()).unwrap();
            inner_result.lock().unwrap().insert(domain, domain_info);
        });
        handles.push(handle);
    }

    //Join Handles
    for handle in handles {
        handle.join().unwrap();
    }

    //Pretty print results in json format
    println!("{}", serde_json::to_string_pretty(&*result.lock().unwrap()).unwrap());

    Ok(())
}