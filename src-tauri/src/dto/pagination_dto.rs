use serde::Serialize;

#[derive(Serialize)]
pub struct Pagination<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub current_page: i16,
    pub size: i16,
    pub total_pages: i64,
}