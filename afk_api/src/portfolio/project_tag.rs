use utils;
use portfolio;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ProjectTag {
    pub name: String,
    pub id: i32,
}

pub fn get_project_tags() -> Vec<ProjectTag> {
    let url = format!("{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "tag/");
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}

pub fn get_project_tag(id: i32) -> ProjectTag {
    let url = format!("{}{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "tag/",
                      id);
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}
