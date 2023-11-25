use actix_web::{HttpResponse, Responder, post, get, web::{Redirect, Data}};
use std::{fs, sync::{Mutex, mpsc}};
use crate::WebSenders;

const TEN : u8 = 10; 

#[get("/")]
pub async fn get_index() -> impl Responder {
    let index: String = fs::read_to_string("index.html").unwrap();

    HttpResponse::Ok().body(index)
}

#[post("/")]
pub async fn receive_data(req_body: String, data : Data<WebSenders>) -> Redirect{
    let parts:Vec<&str>  = req_body.split('&').collect();
    let mut results: Vec<&str>= Vec::<&str>::new();

    for query in parts {
        let data:Vec<&str> = query.split("=").collect();
        let res: &str = *data.get(1).unwrap();
        results.push(res);
    }

    let servo_level: u8 = 'check: {
        let num: &[u8] = results.get(1).unwrap().as_bytes();

        'first: {
            if num.len() == 1 {
                break 'check num.get(0).unwrap() - 48;
            }

            break 'first;
        }

        'second: {
            if num.len() == 0 {
                break 'check 0;
            }

            break 'second;
        }

        let mut final_digit: u8 = 0;
        let mut results_len: u8 = num.len() as u8 - 1;

        for r in num {

            if results_len == 0{
                let holder: u8 = *r - 48;
                final_digit += holder;

                break 'check final_digit
            }

             let holder: u8 = (*r - 48) * (TEN.pow( results_len as u32));
             final_digit += holder;
             results_len -= 1;
         }

        break 'check final_digit
    };

    let led_status = *results.get(2).unwrap();
    let send_value: bool = led_status == "on";

    data.servo_sender
        .lock()
        .unwrap()
        .send(servo_level)
        .expect("Couldnt send the servo value");

    data.led_sender
        .lock()
        .unwrap()
        .send(send_value)
        .expect("Couldnt send the led value");

    Redirect::to("/").see_other()
}