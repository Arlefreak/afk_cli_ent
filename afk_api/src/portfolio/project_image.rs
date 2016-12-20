use utils;
use portfolio;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct ProjectImage {
    pub id: i32,
    pub thumbnail: String,
    pub thumbnailBW: String,
    pub order: i32,
    pub publish: bool,
    pub name: String,
    pub caption: String,
    pub image: String,
    pub imgType: String,
    pub dateCreated: String,
    pub project: i32,
}

pub fn get_project_images() -> Vec<ProjectImage> {
    let url = format!("{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "projectsImages/");
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}

pub fn get_project_image(id: i32) -> ProjectImage {
    let url = format!("{}{}{}{}",
                      utils::API_ENDPOINT,
                      portfolio::API_ENDPOINT,
                      "projectsImages/",
                      id);
    let incoming_request = utils::get::content(&url).unwrap();
    json::decode(&incoming_request).unwrap()
}
