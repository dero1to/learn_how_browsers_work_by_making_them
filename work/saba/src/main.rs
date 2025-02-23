#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

extern crate alloc;

use alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() {
    let client = HttpClient::new();
    match client.get("host.test".to_string(), 8000, "/test.html".to_string()) {
        Ok(res) => {
            println!("response:\n{:#?}", res);
        }
        Err(e) => {
            println!("error:\n{:#?}", e);
        }
    }
    0;
}

entry_point!(main);
