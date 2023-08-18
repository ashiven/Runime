use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct QuoteModel {
    pub id: i32,
    pub quote: Option<String>,
    pub category: Option<String>,
    pub anime: Option<String>,
    pub character: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct QuoteModelResponse {
    pub id: String,
    pub quote: String,
    pub category: String,
    pub anime: String,
    pub character: String,
}
