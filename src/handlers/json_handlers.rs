use actix_web::{web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    name: String,
}

#[derive(Serialize)]
pub struct Resp {
    name: String,
    has_3_or_more_letters: bool
}

pub async fn json(json: Json<Info>) -> impl Responder {
    let resp = Resp {
        name: json.name.clone(),
        has_3_or_more_letters: if json.name.len() > 2 {true} else {false}
    };

    HttpResponse::Ok().json(resp)
}