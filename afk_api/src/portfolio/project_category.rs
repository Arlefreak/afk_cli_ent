use utils;
use portfolio;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct ProjectCategory {
    pub id: i32,
    pub order: i32,
    pub name: String,
    pub slug: String,
    pub smallDescription: String,
    pub description: String,
    pub image: Option<String>,
}

pub fn get_project_categories() -> Vec<ProjectCategory> {
    let url = format!("{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "projectsCategories/");
    let incoming_request = utils::get::content(&url).unwrap();
    let decoded: Vec<ProjectCategory> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_category(id: i32) -> ProjectCategory {
    let url = format!("{}{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "projectsCategories/",
                      id);
    let incoming_request = utils::get::content(&url).unwrap();
    let decoded: ProjectCategory = json::decode(&incoming_request).unwrap();
    decoded
}
