# URL-Shortener-With-Analytics

The following project is written in **Rust** using **Actix-Web** and **SQLx + SQLite**, it provides both a RESTful API and a simple web interface using **HTML**, **CSS** and **JavaScript**.

**Status**

Development in Progress...

- Implemented: MVP (shorten with and without custom slug and redirect URLs), Analytics (qr code generator, expiration date, clicks, unique visitors, geographic location), Requirements (RESTful API with endpoints for shortening, redirecting and analytics, short code generation [6-8 characters, alphanumeric], rate limiter, DB for URLs and click events, collision detection and handling), Authentication
- Soon to be implemented: Requirements (web dashboard showing all links and statistics), Fun Stuff(filtering, charts, slug trends)

## Description

**URL Shortener with Analytics** is a full-stack Rust project that transforms long URLs into shorter, shareable URLs while collecting basic analytics. 

Users can shorten links, create custom slugs, view different statistics (total clicks, unique visitors, geographic location), generate QR codes and set expiration dates.  

The project demonstrates key backend concepts such as HTTP request handling, database management and asynchronous programming. All of this can be found in a single, lightweight service.

### Why This Project?

URL Shorteners with Analytics address many real-world engineering challenges:
- Preventing slug collisions;  
- Handling high traffic efficiently;  
- Ensuring database integrity and security;  
- Presenting analytics data clearly.  

It’s a complete **end-to-end web service** built entirely with Rust.

## System Architecture

The chosen architecture follows a **modular**, **layered design** that separates concerns between the web interface, API logic, database layer, and storage.  
This makes the project easier to understand, extend, and maintain.

This architecture strikes a balance between **simplicity** and **scalability**, making it ideal for small to medium web services, educational projects, or prototypes that could later evolve into production grade systems.

<p align="center">
  <img width="535" height="344" alt="ATAD Architecture" src="https://github.com/user-attachments/assets/a37308be-2b03-4712-931b-b6fae5cd1e0b" />
</p>

### Components

- **Web Client:**  
  Simple HTML/CSS/JavaScript frontend used to submit URLs and display the shortened URLs and analytics.

- **API Layer (Actix-Web):**  
  Exposes REST endpoints for:
  - `POST /shorten` -> Creates a shortened URL with an optional custom slug and expiration date;
  - `GET /{slug}` -> Redirects to the original URL using a slug and records analytics data;
  - `GET /stats/{slug}` → Returns analytics for a shortened URL (expiration date, clicks, unique visitors, geolocation stats);
  - `GET /qr/{slug}` → Generates a QR code for the shortened URL;
  - `GET /` -> Serves the web dashboard page.
  
  All endpoints are protected by a simple in-memory rate limiter (10 requests per minute per IP address).

- **Services (Logical Components)::**
  - `URL Service`-> Handles URL creation, storage, retrieval and expiration validation using the database;
  - `Analytics Service` -> Tracks and fetches statistics (expiration dates, QR code generator, clicks, unique visitors, geographic location);  
  - `QR Code Service` -> Generates QR codes for each shortened URL;
  - `Rate Limiter` -> Restricts the number of requests per IP address within a defined time window.

- **Database (SQLx + SQLite):**
  - `urls` table -> Stores expiration dates, slugs and original URLs;
  - `clicks` table -> Stores clicks' analytics data.  

## Technologies Used (so far)

- [Rust](https://www.rust-lang.org/)
- [Actix-Web](https://actix.rs/)
- [SQLx](https://github.com/launchbadge/sqlx)
- [SQLite](https://sqlite.org/)
- [Rand](https://docs.rs/rand/latest/rand/)
- [Webbrowser](https://docs.rs/webbrowser/latest/webbrowser/)
- [Tokio](https://tokio.rs/)
- [Serde](https://serde.rs/)
- [Serde JSON](https://docs.rs/serde_json/)
- [Chrono](https://docs.rs/chrono/)
- [Reqwest](https://docs.rs/reqwest/)
- [QR Code](https://docs.rs/qrcode/)
- [Image](https://docs.rs/image/)
- [Once Cell](https://docs.rs/once_cell/)
- [Dirs](https://docs.rs/dirs/)

Make sure Rust and Cargo are installed on your system. Verify using:

```bash
rustc --version
rustup --version
cargo --version
```

If you do not have them installed, open your terminal and run these commands:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-edit
brew install imagemagick
cargo new Proiect_ATAD --bin
```

## Installation

### 1. Cloning the Project

```bash
git clone https://github.com/Iasmina012/URL-Shortener-With-Analytics
```

### 2. Running the Project

```bash
cd <repository_name>
cargo clean
cargo build
cargo run
```

The application will start locally at: http://127.0.0.1:8080/

The browser should open automatically. If not, open the URL manually.