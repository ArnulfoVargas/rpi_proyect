mod  routes;
use actix_web::{HttpServer, App, web::Data };
use std::{thread, sync::Mutex};
use std::sync::mpsc;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let (tx, sx) = mpsc::channel::<u8>();
    let (tx2, sx2) = mpsc::channel::<bool>();
    
    thread::spawn(move || {
        for level in sx{
            println!("{}", level);
        }
    });
    
    thread::spawn(move || {
        for led in sx2{
            println!("{}", led);
        }
    });

    let data: Data<WebSenders> = Data::new(WebSenders { servo_sender: Mutex::new(tx), led_sender: Mutex::new(tx2) });

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

pub struct WebSenders {
    servo_sender: Mutex<mpsc::Sender<u8>>,
    led_sender  : Mutex<mpsc::Sender<bool>>,
}