use serde::{Deserialize, Serialize};

//#[derive(Deserialize, Debug)]
//pub struct FilterOptions {
//    pub page: Option<usize>,
//    pub limit: Option<usize>,
//}

#[derive(Deserialize, Debug)]
pub struct ParameterOptions {
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateQuoteSchema {
    pub quote: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateQuoteSchema {
    pub quote: Option<String>,
    pub category: Option<String>,
}
