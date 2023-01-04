extern crate handlebars;

mod api;

use api::Routes;

use actix_web::{
    App,
    web,
    HttpServer,
};
use actix_files::Files;
use handlebars::Handlebars;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Setting up handlebars
    let mut handlebars = Handlebars::new();
    match handlebars.register_templates_directory(".hbs", "./templates/") {
        Ok(v) => println!("main: {:?}",v),
        Err(e) => println!("main: {:?}", e)
    }
    let hbars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(hbars_ref.clone())
            .service(Files::new("/assets", "./static"))
            .service(Routes::index)
            .default_service(web::route().to(Routes::_404))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
