#![allow(dead_code)]
extern crate hyper;
extern crate rustc_serialize;

use afk::api;
use rustc_serialize::{json};

pub static API_ENDPOINT: &'static str = "diary/posts/";

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct Post {
    pub id: i32,
    pub tags: Vec<String>,
    pub order: i32,
    pub publish: bool,
    pub title: String,
    pub slug: String,
    pub text: String,
    pub dateCreated: String,
    pub dateUpdated: String,
}

pub fn get_posts() -> Vec<Post>{
    let url = format!("{}{}", api::API_ENDPOINT, API_ENDPOINT);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Vec<Post> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_post(id:  i32) -> Post{
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Post = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn log_post(id: i32){
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Post = json::decode(&incoming_request).unwrap();
    println!("{:?}", decoded.title);
}

pub fn diary(){
    println!("called `afk::diary()`");
}
