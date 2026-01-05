use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use std::{thread, time::Duration, io::Cursor, collections::HashMap, sync::Mutex};
use webbrowser;
use sqlx::{SqlitePool, Row};
use rand::{thread_rng, Rng, distributions::Alphanumeric};
use qrcode::QrCode;
use image::{Luma, ImageOutputFormat};
use chrono::{NaiveDate, Utc};
use once_cell::sync::Lazy;

mod database;
use database::{init_db, insert_url, get_url, record_click, get_click_stats};
//use database::{reset_db};

static RATE_LIMITER: Lazy<Mutex<HashMap<String, Vec<i64>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
const MAX_REQUESTS: usize = 10;
const WINDOW_SECONDS: i64 = 60;

fn check_rate_limit(ip: &str) -> bool {

    let now = Utc::now().timestamp();

    let mut map = RATE_LIMITER.lock().unwrap();
    let entries = map.entry(ip.to_string()).or_insert(Vec::new());

    //delete old requests
    entries.retain(|&t| now - t < WINDOW_SECONDS);

    if entries.len() >= MAX_REQUESTS {
        return false;
    }

    entries.push(now);
    true

}

fn get_client_ip(req: &HttpRequest) -> String {

    if let Some(forwarded) = req.headers().get("X-Forwarded-For") {
        if let Ok(ip) = forwarded.to_str() {
            return ip.split(',').next().unwrap_or(ip).to_string();
        }
    }

    req.peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())

}

#[get("/")]
async fn index() -> impl Responder {

    let html = include_str!("index.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)   

} 

#[post("/shorten")]
async fn shorten(pool: web::Data<SqlitePool>, json: web::Json<serde_json::Value>, req: HttpRequest) -> impl Responder {

    let ip = get_client_ip(&req);

    if !check_rate_limit(&ip) {
        return HttpResponse::TooManyRequests().json(
            serde_json::json!({ "error": "Too many requests! Please slow down."})
        );
    }

    let url = match json.get("url").and_then(|v| v.as_str()) {
        Some(u) if !u.trim().is_empty() => u.trim(),
        _ => return HttpResponse::BadRequest().json(serde_json::json!({ "error": "Missing or invalid 'url' field" })),
    };

    let optional_slug = json.get("slug").and_then(|v| v.as_str()).map(|s| s.trim()).filter(|s| !s.is_empty());

    let final_slug = match optional_slug {
        Some(s) => s.to_string(),
        None => thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect()
    };

    let expires_optional = json.get("expires").and_then(|v| v.as_str()).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

    if let Some(ref ex) = expires_optional {
        if NaiveDate::parse_from_str(ex, "%Y-%m-%d").is_err() {
            return HttpResponse::BadRequest().json(serde_json::json!({ "error": "Invalid date format for 'expires'. Use YYYY-MM-DD" }));
        }
    }

    match insert_url(&pool, &final_slug, url, expires_optional.clone()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "short_url": format!("http://localhost:8080/{}", final_slug),
            "expires": expires_optional
        })),
        Err(e) => {
            let msg = format!("{}", e);
            if msg.contains("UNIQUE") || msg.contains("unique") {
                HttpResponse::BadRequest().json(serde_json::json!({ "error": "Slug already exists!" }))
            } else {
                HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Database error" }))
            }
        }
    }

}

#[get("/{slug}")]
async fn redirect(slug: web::Path<String>, pool: web::Data<SqlitePool>, req: HttpRequest) -> impl Responder {
    
    let ip = get_client_ip(&req);

    if !check_rate_limit(&ip) {
        return HttpResponse::TooManyRequests()
            .body("Too many requests! Please slow down.");
    }

    let slug = slug.into_inner();

    match get_url(&pool, &slug).await {
        
        Ok(Some((url, expires_opt))) => {
            //checks expiration server-side
            if let Some(exp_str) = expires_opt {
                if let Ok(exp_date) = NaiveDate::parse_from_str(&exp_str, "%Y-%m-%d") {
                    if exp_date < chrono::Utc::now().date_naive() {
                        return HttpResponse::Gone().body("This link has expired");
                    }
                }
            }

            //saves click
            let ip = req.peer_addr().map(|a| a.ip().to_string());
            let ua = req.headers()
                .get("User-Agent")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            record_click(&pool, &slug, ip, ua).await;

            HttpResponse::Found()
                .append_header(("Location", url))
                .finish()
        }
        Ok(None) => HttpResponse::NotFound().body("Short URL not found"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }

}

#[get("/stats/{slug}")]
async fn stats(slug: web::Path<String>, pool: web::Data<SqlitePool>) -> impl Responder {
    
    let clicks = get_click_stats(&pool, &slug).await;

    HttpResponse::Ok().json(serde_json::json!({
        "slug": slug.into_inner(),
        "total_clicks": clicks
    }))
    
}

#[get("/qr/{slug}")]
async fn generate_qr(slug: web::Path<String>) -> impl Responder {

    let short_url = format!("http://localhost:8080/{}", slug);

    let code = QrCode::new(short_url.as_bytes()).unwrap();

    let image = code.render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .build();

    let mut buffer = Cursor::new(Vec::new());
    image
        .write_to(&mut buffer, ImageOutputFormat::Png)
        .unwrap();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(buffer.into_inner())
        
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
            .service(generate_qr)
            .service(stats)
    })
    .bind((host, port))?
    .run()
    .await

}