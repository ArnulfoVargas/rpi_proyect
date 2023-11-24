mod  routes;
use actix_web::{HttpServer, App, web::{self, Data} };
use std::{thread, sync::Mutex};
use std::sync::mpsc;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let (tx, sx) = mpsc::channel::<u8>();
    
    thread::spawn(move || {
        for data in sx{
            println!("{}", data);
        }
    });
    
    let data: Data<Mutex<mpsc::Sender<u8>>> = Data::new(Mutex::new(tx));

    HttpServer::new(move || {
        App::new()
        .app_data(Data::clone(&data))
        .service(routes::get_index)
        .service(routes::receive_data)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}