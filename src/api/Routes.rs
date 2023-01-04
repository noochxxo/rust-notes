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

    let page_data = json!({
        "name": "eric",
        "title": "Ugly ass Rust Notes app"
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