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

    let servo_pwm = Pwm::new(Channel::Pwm0).unwrap();
    servo_pwm.set_frequency(50, 1);
    servo_pwm.enable();

    let (tx, sx) = mpsc::channel::<u8>();
    let (tx2, sx2) = mpsc::channel::<bool>();
    
    thread::spawn(move || {
        for level in sx{
            servo_pwm.set_duty_cycle((level as f64) / 12 );
        }
    });
    
    thread::spawn(move || {
        for is_turned in sx2{
            if is_turned{
                led.set_high();
            }
            else {
                led.set_low();
            }
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