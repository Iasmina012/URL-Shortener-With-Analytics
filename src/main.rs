use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::thread;
use std::time::Duration;
use webbrowser;

#[get("/")]
async fn index() -> impl Responder {
    let html = include_str!("index.html"); // CITEȘTE index.html
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let host = "127.0.0.1";
    let port = 8080;
    let url = format!("http://{}:{}/", host, port);

    thread::spawn({
        let url = url.clone();
        move || {
            thread::sleep(Duration::from_secs(1));
            if webbrowser::open(&url).is_ok() {
                println!("Browser opened at {}", url);
            } else {
                println!("Couldn't open browser. Open manually: {}", url);
            }
        }
    });

    println!("Server started at {}", url);
    HttpServer::new(|| App::new().service(index))
        .bind((host, port))?
        .run()
        .await

}