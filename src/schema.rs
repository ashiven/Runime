use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParameterOptions {
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateQuoteSchema {
    pub quote: String,
    pub category: String,
    pub anime: String,
    pub character: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateQuoteSchema {
    pub quote: Option<String>,
    pub category: Option<String>,
    pub anime: Option<String>,
    pub character: Option<String>,
}
