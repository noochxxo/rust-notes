use crate::utils::Serialization::{
    notes_from_file,
    Note,
    };


use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
    };
use serde_json::json;
use handlebars::Handlebars;

#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {

    let filename = String::from("./notes/data.json");

    let notes = notes_from_file(&filename);
    
    let page_data = json!({
        "name": "Eric Trotchie",
        "title": "Ugly ass Rust Notes app",
        "data": notes,
        });

    
    let body: String = match hb.render("views/index", &page_data ) {
        Ok(v) => v,
        Err(e) => "".to_owned(),
    };
    HttpResponse::Ok().body(body)
}



pub async fn _404() -> impl Responder {
    HttpResponse::Ok().body("You made a wrong turn, muh guy.")
}