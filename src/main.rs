use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{thread, time::Duration};
use webbrowser;
use sqlx::SqlitePool;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

mod database;
use database::{init_db, insert_url, get_url};
//use database::{reset_db};

#[get("/")]
async fn index() -> impl Responder {

    let html = include_str!("index.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

} 

#[post("/shorten")]
async fn shorten(pool: web::Data<SqlitePool>, json: web::Json<serde_json::Value>) -> impl Responder {
    
    let url = json["url"].as_str().unwrap();
    let slug = json["slug"].as_str().unwrap_or("");

    let final_slug = if slug.is_empty() {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    } else {
        slug.to_string()
    };

    match insert_url(&pool, &final_slug, url).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "short_url": format!("http://localhost:8080/{}", final_slug)
        })),
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Slug already exists!"
        }))
    }

}

#[get("/{slug}")]
async fn redirect(slug: web::Path<String>, pool: web::Data<SqlitePool>) -> impl Responder {
    
    if let Some(url) = get_url(&pool, &slug).await {
        return HttpResponse::Found()
            .append_header(("Location", url))
            .finish();
    }
 
    HttpResponse::NotFound().body("Short URL not found")

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = init_db().await;

    //when you need to reset db
     //reset_db(&pool)
        //.await
        //.expect("Failed to reset database");

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
 
    println!("Server running at {}", url);

   HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(shorten)
            .service(redirect)
    })
    .bind((host, port))?
    .run()
    .await

}