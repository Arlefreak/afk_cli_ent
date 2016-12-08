use utils;
use portfolio;
use portfolio::project_tag;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct Project {
    pub id: i32,
    pub tags: Vec<project_tag::ProjectTag>,
    pub order: i32,
    pub publish: bool,
    pub name: String,
    pub slug: String,
    pub smallDescription: String,
    pub description: String,
    pub dateCreated: String,
    pub dateUpdated: String,
    pub category: i32,
}

pub fn get_projects() -> Vec<Project> {
    let url = format!("{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "projects/");
    let incoming_request = utils::get::content(&url).unwrap();
    let decoded: Vec<Project> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project(id: i32) -> Project {
    let url = format!("{}{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "projects/",
                      id);
    let incoming_request = utils::get::content(&url).unwrap();
    let decoded: Project = json::decode(&incoming_request).unwrap();
    decoded
}
