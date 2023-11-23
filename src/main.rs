mod  routes;
use actix_web::{HttpServer, App, web};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(routes::get_index))
        .service(routes::receive_data)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
