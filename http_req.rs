extern crate hyper;
extern crate serde_json;
use std::io::Read;
use std::collections::HashMap;
use hyper::client::Client;

fn main(){
    let client = Client::new();
    let mut res = client.get("http://httpbin.org/ip").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let json: HashMap<String, String> = serde_json::from_str(&body).unwrap();
    println!("{:#?}", json);
}
