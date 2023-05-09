use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(FromRow, Serialize, Deserialize)]
pub struct Pagination {
    pub next: DateTime<chrono::Utc>,
    pub limit: usize,
}

// #[derive(Serialize)]
// pub struct PaginationContext {
//     pub next: i64,
//     pub limit: usize,
// }

// impl Pagination for PaginationContext {
//     pub fn to_context(&self) ->
// }