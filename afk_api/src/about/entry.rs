use utils;
use about;
use rustc_serialize::{json};

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
    let url = format!("{}{}", utils::API_ENDPOINT, about::API_ENDPOINT);
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}

pub fn get_entry(id:  i32) -> Entry{
    let url = format!("{}{}{}", utils::API_ENDPOINT, about::API_ENDPOINT, id);
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}

pub fn log_entry(id: i32){
    println!("{:?}", get_entry(id).slug);
}
