use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseInfo {
    pub http_status: u16,
    pub cert_valid: bool,
    pub layer_0_version: Option<String>, 
    pub layer_0_timings: Option<HashMap<String, u16>>,
    pub ips: Vec<std::net::IpAddr>
}

impl Default for ResponseInfo {
    fn default() -> Self { 
        ResponseInfo{
            http_status: 0,
            cert_valid: false,
            layer_0_version: None, 
            layer_0_timings: None,
            ips: vec![]
        }
    }
}