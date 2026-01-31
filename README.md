# URL-Shortener-With-Analytics

The following project is written in **Rust** using **Actix-Web** and **SQLx + SQLite**. It provides both a RESTful API and a simple web interface using **HTML**, **CSS** and **JavaScript**. The user authentication process is handled through **Firebase**.

**Status:** DONE

- `MVP` (shorten URLs with/without custom slug and redirect them);
- `Analytics` (QR code generator, expiration date, clicks, unique visitors, geographic location);
- `Requirements` (RESTful API with endpoints for shortening, redirecting and analytics, short code generation [6-8 characters, alphanumeric], rate limiter, DB for URLs and click events, collision detection and handling, web dashboard showing all URLs and statistics);
- `Firebase Authentication`
- `Search, Filter & Sort URLs` that were created across the app (without the expired ones)

## Description

**URL Shortener with Analytics** is a full-stack web application written in Rust that transforms long URLs into short, easy to share URLs while collecting and providing different analytics.

Authenticated users can create shorter URLs with optional custom slugs and expiration dates, view and manage all their previously created URLs and access per-URL analytics such as total clicks, unique visitors and geographic distribution of visitors. Each shortened URL also supports automatic QR code generation, which can be previewed and downloaded directly from the interface.

The application includes a complete URL management system where users can view all of their generated URLs in a dashboard, inspect statistics for each one on demand and delete URLs when they are no longer needed. In addition, a dedicated Trends tab provides a global overview of URLs' performance, allowing users to explore trends across all URLs such as search by slug, filter popular URLs based on visitors and clicks activity, sort results by slug, expiration date or number of clicks and visitors.

The project demonstrates key backend concepts such as RESTful API design, authentication using Firebase, HTTP request handling, rate limiting, collision detection and handling, database management and asynchronous programming. Analytics collection and retrieval are handled asynchronously to ensure performance and scalability. All of this can be found in a single, lightweight service.

### Why This Project?

URL Shorteners with Analytics address many real-world engineering challenges:
- Integrating a frontend interface with a RESTful backend in a clean architecture;
- Implementing authentication and authorization to protect user resources;
- Ensuring database integrity, consistency and security;
- Preventing slug collisions while maintaining short, readable URLs;
- Protecting the service from abuse using rate limiting mechanisms;
- Handling high traffic efficiently and safely through asynchronous request processing;
- Designing and collecting analytics data such as click events, unique visitors and geographic distribution;
- Presenting analytics data clearly and intuitively through a web dashboard;
- Managing URLs lifecycle through deletion.

## System Architecture

The chosen architecture follows a **modular**, **layered design** that separates concerns between the web interface, API logic, database layer, and storage.
This makes the project easier to understand, extend, and maintain.

This architecture strikes a balance between **simplicity** and **scalability**, making it ideal for small to medium web services, educational projects, or prototypes that could later evolve into production grade systems.

<p align="center">
  <img width="741" height="390" alt="ATAD Final Architecture" src="https://github.com/user-attachments/assets/2a20cc86-76c1-4d18-a2b4-315c9658feca" />
</p>

### Components

- **Web Client:**
  Simple HTML/CSS/JavaScript frontend used to submit URLs and display the shortened URLs and analytics.

- **API Layer (Actix-Web):**
  Exposes REST endpoints for:
  - `POST /shorten` -> Creates a shortened URL with an optional custom slug and expiration date (random slugs retried up to 5 times);
  - `GET /{slug}` -> Redirects to the original URL using a slug and records analytics data;
  - `GET /stats/{slug}` -> Returns analytics for a shortened URL (expiration date, clicks, unique visitors, geolocation stats);
  - `GET /qr/{slug}` -> Generates a QR code for the shortened URL;
  - `GET /my_urls` -> Returns user's URLs (requires Firebase authentication);
  - `DELETE /url/{slug}` -> Deletes URLs (requires ownership and authentication);
  - `GET /` -> Serves the web dashboard page.

  All endpoints are protected by a simple in-memory rate limiter (10 requests per minute per IP address).

- **Services (Logical Components):**
  - `URL Shorten Service`-> Handles URL creation, storage, retrieval and expiration validation using the database;
  - `Analytics Service` -> Tracks and fetches statistics (expiration dates, QR code generator, clicks, unique visitors, geographic location);
  - `QR Code Service` -> Generates QR codes for each shortened URL;
  - `Rate Limiter` -> Restricts the number of requests per IP address within a defined time window.

- **Database (SQLx + SQLite):**
  - `urls` table -> Stores expiration dates, slugs and original URLs;
  - `clicks` table -> Stores clicks' analytics data.

## Technologies Used

### Backend & Runtime
- **[Rust](https://www.rust-lang.org/)** – systems programming language focused on safety, speed, and concurrency
- **[Actix Web](https://actix.rs/)** – high-performance, asynchronous web framework for Rust
- **[Tokio](https://tokio.rs/)** – asynchronous runtime for non-blocking I/O

### Database & Storage
- **[SQLx](https://github.com/launchbadge/sqlx)** – async, compile-time checked SQL toolkit
- **[SQLite](https://sqlite.org/)** – lightweight, embedded relational database
- **[Dirs](https://docs.rs/dirs/)** – access to platform-specific standard directories

### Networking & Authentication
- **[JSON Web Token](https://docs.rs/jsonwebtoken/)** – JWT-based authentication and authorization
- **[Reqwest](https://docs.rs/reqwest/)** – async HTTP client for Rust
- **[Webbrowser](https://docs.rs/webbrowser/latest/webbrowser/)** – open URLs in the system’s default browser
- **[Ngrok](https://ngrok.com/)** – secure tunneling for exposing local servers to the public internet

### Serialization & Utilities
- **[Serde](https://serde.rs/)** – serialization and deserialization framework
- **[Serde JSON](https://docs.rs/serde_json/)** – JSON support for Serde
- **[Once Cell](https://docs.rs/once_cell/)** – lazy static initialization
- **[Chrono](https://docs.rs/chrono/)** – date and time handling
- **[Rand](https://docs.rs/rand/)** – random data generation

### Media & Encoding
- **[QR Code](https://docs.rs/qrcode/)** – QR code generation
- **[Image](https://docs.rs/image/)** – image processing and encoding

### External Services
- **[Firebase](https://firebase.google.com/)** – backend services for authentication, hosting, and real-time data

### Frontend
- **[HTML](https://developer.mozilla.org/en-US/docs/Web/HTML)** – markup language for structuring web pages
- **[CSS](https://developer.mozilla.org/en-US/docs/Web/CSS)** – styling and layout for web interfaces
- **[JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript)** – client-side interactivity and logic

## Installation

In order to make your local server publicly accessible, you can use [Ngrok](https://ngrok.com) to expose it. You will need to sign up for an account in order to get a free authentication token!

Make sure Rust, Cargo and Ngrok are installed on your system. Verify using the following commands in a terminal:

```bash
rustc --version
rustup --version
cargo --version
ngrok version
```

If you do not have them installed, open your terminal and run these commands:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-edit
brew install imagemagick
brew install ngrok
ngrok config add-authtoken token
cargo new <project_name> --bin
```

### 1. Cloning the Project

Open your terminal and run the command below:

```bash
git clone https://github.com/Iasmina012/URL-Shortener-With-Analytics
```

### 2. Setting up Firebase Authentication

This project uses **Firebase Authentication (Email/Password)** for user authentication. To run the project locally, you must configure your own Firebase project.

#### 1. Create a Firebase Project
- Go to the [Firebase Console](https://console.firebase.google.com/)
- Create a new project

#### 2. Enable Email/Password Authentication
- In the Firebase Console, navigate to **Authentication -> Sign-in method**
- Enable **Email/Password**

#### 3. Add a Web Application
- In **Project Settings -> General**, add a new **Web Application**
- Copy the Firebase configuration object

#### 4. Configure the Frontend
Replace the Firebase configuration in `index.html` with your own values:

```js
const firebaseConfig = {
  apiKey: "YOUR_API_KEY",
  authDomain: "YOUR_PROJECT_ID.firebaseapp.com",
  projectId: "YOUR_PROJECT_ID",
};
```

### 3. Running the Project

Open a terminal and run Ngrok on the same port as your server:
```bash
ngrok http 8080
```

Ngrok will give you a public URL like:
```bash
Forwarding                    https://unmature-undefectively-kala.ngrok-free.dev
```

Open another terminal and run these commands:
```bash
cd <repository_name>
cargo clean
cargo build
export BASE_URL="https://unmature-undefectively-kala.ngrok-free.dev"
cargo run
```

The application will start locally at: http://127.0.0.1:8080/

The browser should open automatically. If not, open the URL manually.