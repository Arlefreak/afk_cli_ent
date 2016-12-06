#![allow(dead_code)]
extern crate hyper;
extern crate rustc_serialize;
extern crate url;

use std::io::Read;
use hyper::{Client};
use rustc_serialize::{Encodable, json};
use url::form_urlencoded;

pub static API_ENDPOINT: &'static str = "http://api.arlefreak.com/";

pub fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

type Query<'a> = Vec<(&'a str, &'a str)>;

pub fn post_query(url: &str, query: Query) -> hyper::Result<String> {
    let client = Client::new();
    let body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(query.iter())
        .finish();
    let mut response = try!(client.post(url).body(&body[..]).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

pub fn post_json<T>(url: &str, payload: &T) -> hyper::Result<String>
    where T: Encodable {
    let client = Client::new();
    let body = json::encode(payload).unwrap();
    let mut response = try!(client.post(url).body(&body[..]).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}
