mod  routes;
use actix_web::{HttpServer, App, web};
use std::thread;
use std::time::Duration;


#[actix_web::main]
async fn main(){

    let t1 = thread::spawn(|| {
        println!("Running");
        run_app();

        println!("Running");
        thread::sleep(Duration::from_secs(100));
    });

    
    t1.join().unwrap();
    ()
}


async fn run_app() -> std::io::Result<()>{
    println!("Running");
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(routes::get_index))
        .service(routes::receive_data)
    })
    .bind("127.0.0.1:3000")?.run().await
}