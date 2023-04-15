use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Tag {
    pub id: u32,
    pub tagname: String,
    pub description: Option<String>
}

