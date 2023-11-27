use actix_web::{HttpResponse, Responder, post, get, web::{Redirect, Data}};
use std::{fs, sync::{Mutex, mpsc}};
use crate::WebSenders;

const TEN : u8 = 10; 

#[get("/")]
pub async fn get_index() -> impl Responder {
    let index: String = fs::read_to_string("index.html").expect("index not found");

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

    let led_status = *results.get(0).unwrap();
    let send_value: bool = led_status == "on";

    data.led_sender
        .lock()
        .unwrap()
        .send(send_value)
        .expect("Couldnt send the led value");

    Redirect::to("/").see_other()
}