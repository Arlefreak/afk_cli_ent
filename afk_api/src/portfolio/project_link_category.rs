#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct ProjectLinkCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub image: String,
}
