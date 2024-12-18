use actix_web::{http::header::ContentType, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    name: String,
}

#[derive(Serialize)]
pub struct Resp {
    name: String,
    has_letters: Option<bool>
}

pub async fn json(json: Json<Info>) -> impl Responder {
    let mut resp = Resp { name: json.name.clone(), has_letters: None };

    if json.name.len() > 2 {
        resp.has_letters = Some(true);
    }

    let body = serde_json::to_string(&resp).unwrap();

    HttpResponse::Ok().content_type(ContentType::json()).body(body)
}