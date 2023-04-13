use serde::Serialize;
// use uuid::Uuid;
// use strum_macros::{EnumString, Display};

#[derive(Debug, Serialize)]
pub struct Tag {
    pub id: u32,
    pub tagname: String,
    pub description: Option<String>
}

