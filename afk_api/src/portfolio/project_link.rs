use utils;
use portfolio;
use portfolio::project_link_category;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ProjectLink {
    pub id: i32,
    pub category: project_link_category::ProjectLinkCategory,
    pub order: i32,
    pub name: String,
    pub link: String,
    pub project: i32,
}

pub fn get_project_links() -> Vec<ProjectLink> {
    let url = format!("{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "projectsLinks");
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}

pub fn get_project_link(id: i32) -> ProjectLink {
    let url = format!("{}{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "projectsLinks/",
                      id);
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}
