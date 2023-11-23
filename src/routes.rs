use actix_web::{HttpResponse, Responder, post, web::Redirect};
use std::fs;

pub async fn get_index() -> impl Responder {
    let index = fs::read_to_string("index.html").unwrap();

    HttpResponse::Ok().body(index)
}

#[post("/")]
pub async fn receive_data(req_body: String) -> Redirect{
    
    let parts:Vec<&str>  = req_body.split('&').collect();
    let mut results= Vec::<&str>::new();

    for query in parts {
        let data:Vec<&str> = query.split("=").collect();
        let res = *data.get(1).unwrap();
        results.push(res);
    }

    for res in results{
        println!("{}", res);
    }

    Redirect::to("/").see_other()
}