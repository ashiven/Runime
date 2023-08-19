use crate::model::QuoteModelResponse;
use crate::{
    model::QuoteModel,
    schema::{CreateQuoteSchema, UpdateQuoteSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

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
                serde_json::json!({"status": "success", "result": convert_db_record(&quote)});
            HttpResponse::Ok().json(quote_response)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)})),
    }
}

#[patch("/update/{id}")]
async fn update_quote_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateQuoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let quote_id = path.into_inner();
    let result = sqlx::query_as!(
        QuoteModel,
        r#"SELECT * FROM runime.quotes WHERE id = ?"#,
        quote_id
    )
    .fetch_one(&data.db)
    .await;

    let quote = match result {
        Ok(quote) => quote,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(serde_json::json!({"status": "fail", "message": format!("Quote with ID: {} not found", quote_id)}));
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}",e)}));
        }
    };

    let result = sqlx::query(
        r#"UPDATE runime.quotes SET quote = ?, category = ?, anime = ?, character = ?"#,
    )
    .bind(
        body.quote
            .to_owned()
            .unwrap_or_else(|| quote.quote.clone().unwrap()),
    )
    .bind(
        body.category
            .to_owned()
            .unwrap_or_else(|| quote.category.clone().unwrap()),
    )
    .bind(
        body.anime
            .to_owned()
            .unwrap_or_else(|| quote.anime.clone().unwrap()),
    )
    .bind(
        body.character
            .to_owned()
            .unwrap_or_else(|| quote.character.clone().unwrap()),
    )
    .execute(&data.db)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Quote with ID: {} not found", quote_id);
                return HttpResponse::NotFound()
                    .json(serde_json::json!({"status": "fail", "message": message}));
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": message}));
        }
    }

    let updated_quote = sqlx::query_as!(
        QuoteModel,
        r#"SELECT * FROM runime.quotes WHERE id = ?"#,
        quote_id
    )
    .fetch_one(&data.db)
    .await;

    match updated_quote {
        Ok(quote) => {
            let quote_response =
                serde_json::json!({"status": "success", "result": convert_db_record(&quote)});

            HttpResponse::Ok().json(quote_response)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)})),
    }
}

#[delete("/delete/{id}")]
async fn delete_quote_handler(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let quote_id = path.into_inner();
    let result = sqlx::query!(r#"DELETE FROM runime.quotes WHERE id = ?"#, quote_id)
        .execute(&data.db)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Quote with ID: {} not found", quote_id);
                HttpResponse::NotFound()
                    .json(serde_json::json!({"status": "fail", "message": message}))
            } else {
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => {
            let message = format!("Internal server error: {}", e);
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": message}))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(random_quote_handler)
        .service(healthcheck_handler)
        .service(create_quote_handler)
        .service(update_quote_handler)
        .service(delete_quote_handler);
    conf.service(scope);
}
