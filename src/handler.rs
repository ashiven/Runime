use crate::model::QuoteModelResponse;
use crate::{
    model::QuoteModel,
    schema::{CreateQuoteSchema, UpdateQuoteSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, App, HttpResponse, Responder};

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
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "result": res_string}))
}

#[get("/random")]
pub async fn random_quote_handler(data: web::Data<AppState>) -> impl Responder {
    // we use the sqlx query_as! macro to format our query and for input sanitization if we had any parameters here
    // it also outputs the result into our defined struct QuoteModel
    let quote = sqlx::query_as!(
        QuoteModel,
        r#"SELECT * FROM runime.quotes WHERE rand() <= .3 LIMIT 1"#, // query for a random quote as raw string literal
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    // converting the received QuoteModel via convert_db_record ( unwrapping option strings and type conversion )
    let quote_response = convert_db_record(&quote);

    // serialize the quote struct(possible because we derived the trait for it) via the serde_json macro
    let json_response = serde_json::json!({
        "status": "success",
        "result": quote_response
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/create")]
async fn create_quote_handler(
    body: web::Json<CreateQuoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query(
        r#"INSERT INTO runime.quotes (quote, category, anime, character) VALUES (?, ?, ?, ?)"#,
    )
    .bind(body.quote.to_string())
    .bind(body.category.to_string())
    .bind(body.anime.to_string())
    .bind(body.character.to_string())
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string()); // map_err maps a function to a result type that will only execute when the result variant is Err(err) where err is of type sqlx::Error

    if let Err(err) = result {
        if err.contains("Duplicate entry") {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail", "message": "Quote already exists"}));
        }
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)}));
    }

    let result = sqlx::query_as!(
        QuoteModel,
        r#"SELECT * FROM runime.quotes ORDER BY id DESC LIMIT 1"#
    )
    .fetch_one(&data.db)
    .await;

    // when the db operation was successful we get an Ok variant, in which case we want to extract the QuoteModel from that and convert and return it, otherwise we send an internal server error
    match result {
        Ok(quote) => {
            let quote_response =
                serde_json::json!({"status": "success", "response": convert_db_record(&quote)});
            HttpResponse::Ok().json(quote_response)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)})),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(random_quote_handler)
        .service(healthcheck_handler)
        .service(create_quote_handler);
    conf.service(scope);
}
