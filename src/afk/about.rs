#![allow(dead_code)]
extern crate hyper;
extern crate rustc_serialize;

use afk::api;
use rustc_serialize::{json};

pub static API_ENDPOINT: &'static str = "about/entry/";

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct Entry {
    pub id: i32,
    pub order: i32,
    pub publish: bool,
    pub name: String,
    pub slug: String,
    pub text: String,
    pub dateCreated: String,
    pub dateUpdated: String,
}

pub fn get_entries() -> Vec<Entry>{
    let url = format!("{}{}", api::API_ENDPOINT, API_ENDPOINT);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Vec<Entry> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_entry(id:  i32) -> Entry{
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Entry = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn log_entry(id: i32){
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Entry = json::decode(&incoming_request).unwrap();
    println!("{:?}", decoded.name);
}
