#![allow(dead_code)]
extern crate hyper;
extern crate rustc_serialize;

use afk::api;
use rustc_serialize::{json};

pub static API_ENDPOINT: &'static str = "portfolio/";

// TODO: Set Option<> in all optional parameters

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Tag {
    pub name: String,
    pub id: i32,
}

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

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ProjectLink {
    pub id: i32,
    pub category: ProjectLinkCategory,
    pub order: i32,
    pub name: String,
    pub link: String,
    pub project: i32,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ProjectLinkCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub image: String,
}

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

#[derive(RustcDecodable, RustcEncodable, Debug)]
#[allow(non_snake_case)]
pub struct Project {
    pub id: i32,
    pub tags: Vec<Tag>,
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

pub fn get_projects() -> Vec<Project>{
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "projects/");
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Vec<Project> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project(id:  i32) -> Project{
    let url = format!("{}{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "projects/",id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Project = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_categories() -> Vec<ProjectCategory>{
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "projectsCategories/");
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Vec<ProjectCategory> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_category(id:  i32) -> ProjectCategory{
    let url = format!("{}{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "projectsCategories/", id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: ProjectCategory = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_links() -> Vec<ProjectLink>{
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "projectsLinks");
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Vec<ProjectLink> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_link(id:  i32) -> ProjectLink{
    let url = format!("{}{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "projectsLinks/",id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: ProjectLink = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_images() -> Vec<ProjectImage>{
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "projectsImages/");
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Vec<ProjectImage> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_image(id:  i32) -> ProjectImage{
    let url = format!("{}{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "projectsImages/",id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: ProjectImage = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_tags() -> Vec<Tag>{
    let url = format!("{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "tag/");
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Vec<Tag> = json::decode(&incoming_request).unwrap();
    decoded
}

pub fn get_project_tag(id:  i32) -> Tag{
    let url = format!("{}{}{}{}", api::API_ENDPOINT, API_ENDPOINT, "tag/",id);
    let incoming_request = api::get_content(&url).unwrap();
    let decoded: Tag = json::decode(&incoming_request).unwrap();
    decoded
}
