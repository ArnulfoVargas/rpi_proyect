mod  routes;
use actix_web::{HttpServer, App, web::Data };
use std::{thread, sync::Mutex};
use std::sync::mpsc;

use rppal::{gpio::Gpio, pwm::{Channel, Pwm}};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let led_pin:u8 = 15;
    let led_gpio = Gpio::new().unwrap();
    let mut led = led_gpio.get(led_pin).unwrap().into_output();

    let (tx, sx) = mpsc::channel::<bool>();
    
    thread::spawn(move || {
        for is_turned in sx{
            if is_turned{
                led.set_high();
            }
            else {
                led.set_low();
            }
        }
    });

    let data: Data<WebSenders> = Data::new(WebSenders { led_sender: Mutex::new(tx1) });

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
    led_sender: Mutex<mpsc::Sender<bool>>,
}