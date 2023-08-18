use crate::model::QuoteModelResponse;
#[allow(unused_imports)]
use crate::{
    model::QuoteModel,
    schema::{CreateQuoteSchema, UpdateQuoteSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

/*
here we convert QuoteModel to QuoteModelResponse because
the datatypes that we got from the DB are not compatible with our app
and we would like to convert them before further handling
*/
fn convert_db_record(q: &QuoteModel) -> QuoteModelResponse {
    QuoteModelResponse {
        id: q.id.to_string(),
        quote: q.quote.to_owned().unwrap(), // clone the borrowed data via to_owned and unwrap it
        category: q.category.to_owned().unwrap(),
        anime: q.anime.to_owned().unwrap(),
        character: q.character.to_owned().unwrap(),
    }
}

#[get("/healthcheck")]
pub async fn healthcheck_handler() -> impl Responder {
    let res_string = "This is the runime API reporting back in full health!";
    HttpResponse::Ok().json(json!({"status": "success", "result": res_string}))
}

#[get("/quote")]
pub async fn random_quote_handler(data: web::Data<AppState>) -> impl Responder {
    // this is done for input sanitization if we had any input here
    let quote = sqlx::query_as!(
        QuoteModel,
        r#"SELECT * FROM runime.quotes WHERE rand() <= .3 LIMIT 1"#, //query for a random quote as raw string literal
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let quote_response = convert_db_record(quote.get(0).unwrap());

    // serialize the quote struct(possible because we derived the trait for it) via the serde_json macro
    let json_response = serde_json::json!({
        "status": "success",
        "result": quote_response
    });
    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(random_quote_handler);
    conf.service(scope);
}
