use hyper;
use hyper::Client;
use std::io::Read;
use rustc_serialize::{Encodable, json};
use url::form_urlencoded;

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
    where T: Encodable
{
    let client = Client::new();
    let body = json::encode(payload).unwrap();
    let mut response = try!(client.post(url).body(&body[..]).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}
