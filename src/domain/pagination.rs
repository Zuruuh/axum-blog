use super::serde::empty_string_as_none;

#[derive(Debug, serde::Deserialize)]
pub struct PaginationOptions {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub skip: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub take: Option<i32>,
}
