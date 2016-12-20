use utils;
use diary;
use rustc_serialize::{json};

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
    let url = format!("{}{}", utils::API_ENDPOINT, diary::API_ENDPOINT);
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}

pub fn get_post(id:  i32) -> Post{
    let url = format!("{}{}{}", utils::API_ENDPOINT, diary::API_ENDPOINT, id);
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}

pub fn log_post(id: i32){
    let url = format!("{}{}{}", utils::API_ENDPOINT, diary::API_ENDPOINT, id);
    let incoming_request = utils::get::content(&url).unwrap();
    let decoded: Post = json::decode(&incoming_request).unwrap();
    println!("{:?}", decoded.title);
}

pub fn diary(){
    println!("called `afk::diary()`");
}
