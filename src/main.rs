use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::thread;
use std::time::Duration;
use webbrowser;

#[get("/")]
async fn index() -> impl Responder {

    let html = r#"<!doctype html>
<html lang="ro">

<head>

  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">

  <title>URL Shortener Demo</title>

  <link rel="icon" type="image/png" href="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1200px-Rust_programming_language_black_logo.svg.png">

  <style>

    *, *::before, *::after {
      box-sizing: border-box;
    }

    body {
      font-family: Arial, Helvetica, sans-serif;
      display: flex;
      align-items: center;
      justify-content: center;
      height: 100vh;
      margin: 0;
      background: #f7f7f9;
    }

    .card {
      background: white;
      padding: 2rem 3rem;
      width: 600px; /* mai lat acum */
      border-radius: 12px;
      box-shadow: 0 8px 30px rgba(0,0,0,0.08);
    }

    h1 {
      margin: 0 0 1.2rem 0;
      font-size: 1.6rem;
      color: #111827;
      text-align: center;
    }

    label {
      font-size: 0.9rem;
      color: #374151;
      display: block;
      margin-bottom: 4px;
    }

    input, button {
      width: 100%;
      padding: 12px;
      margin: 8px 0 18px 0;
      font-size: 0.9rem;
      border-radius: 8px;
      line-height: 1.2;
    }

    input {
      border: 1px solid #cbd5e1;
    }

    button {
      background: #2563eb;
      border: none;
      color: white;
      cursor: pointer;
      transition: background 0.2s ease-in-out;
    }

    button:hover {
      background: #1e4fcf;
    }
  </style>

</head>

<body>

  <div class="card">

    <h1>URL Shortener with Analytics</h1>

    <form action="/shorten" method="POST">

      <label>URL to shorten:</label>
      <input type="url" name="url" placeholder="https://example.com/rust-project" required>

      <label>Custom Slug (optional):</label>
      <input type="text" name="slug" placeholder="e.g.: link-name">

      <label>Expiration Date (optional):</label>
      <input type="date" name="expires">

      <button type="submit">Generate a shorter URL</button>

    </form>

  </div>

</body>

</html>"#;

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