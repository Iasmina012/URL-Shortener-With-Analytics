use sqlx::{SqlitePool, Row};
use dirs::home_dir;
use std::{fs, path::PathBuf};

pub async fn init_db() -> SqlitePool {

    let mut db_folder: PathBuf = home_dir().expect("Cannot find HOME directory");

    //create folder
    db_folder.push(".url_shortener");
    println!("Database folder resolved to: {}", db_folder.display());

    if !db_folder.exists() {
        println!("Folder does NOT exist. Trying to create one...");
        fs::create_dir_all(&db_folder).expect("Failed to create database folder");
    }

    //databse file
    db_folder.push("urls.db");

    //NU STERGE mode=rwc
    let db_url = format!("sqlite:{}?mode=rwc", db_folder.to_string_lossy());
    println!("Using database URL: {}", db_url);

    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Couldn't connect to SQLite database");

    //urls table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT UNIQUE NOT NULL,
            url TEXT NOT NULL,
            expires_at TEXT,
            user_id TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");
    
    //backward compatibility
    let columns = sqlx::query("PRAGMA table_info(urls);")
        .fetch_all(&pool)
        .await
        .unwrap();

    //ensure column exists for older DBs (that didn't have expires_at before)
    let has_expires = sqlx::query("PRAGMA table_info(urls);")
        .fetch_all(&pool)
        .await
        .expect("Failed to read table info")
        .into_iter()
        .any(|r| r.get::<String, _>("name") == "expires_at");
    if !has_expires {
        //add column if it does not exist
        let _ = sqlx::query("ALTER TABLE urls ADD COLUMN expires_at TEXT;")
            .execute(&pool)
            .await;
    }

    let has_user_id = columns.iter().any(|c| c.get::<String, _>("name") == "user_id");
    if !has_user_id {
        let _ = sqlx::query("ALTER TABLE urls ADD COLUMN user_id TEXT NOT NULL DEFAULT 'legacy';")
            .execute(&pool)
            .await;
    }

    //clicks table
    sqlx::query(
    r#"
        CREATE TABLE IF NOT EXISTS clicks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT NOT NULL,
            ip TEXT,
            user_agent TEXT,
            country TEXT,
            clicked_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    println!("Database initialized successfully");
    pool

}

pub async fn insert_url(pool: &SqlitePool, slug: &str, url: &str, expires_at: Option<String>, user_id: &str,) -> sqlx::Result<()> {

    sqlx::query("INSERT INTO urls (slug, url, expires_at, user_id) VALUES (?, ?, ?, ?)")
        .bind(slug)
        .bind(url)
        .bind(expires_at)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())

}

pub async fn get_url(pool: &SqlitePool, slug: &str) -> Result<Option<(String, Option<String>)>, sqlx::Error> {

    let row = sqlx::query("SELECT url, expires_at FROM urls WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await?;

    if let Some(r) = row {
        let url: String = r.get("url");
        let expires_at: Option<String> = r.get::<Option<String>, _>("expires_at");
        Ok(Some((url, expires_at))) //if found
    } else {
        Ok(None)
    }

}

pub async fn record_click(pool: &SqlitePool, slug: &str, ip: Option<String>, ua: Option<String>, country: Option<String>) {
    
    let _ = sqlx::query("INSERT INTO clicks (slug, ip, user_agent, country) VALUES (?, ?, ?, ?)")
        .bind(slug)
        .bind(ip)
        .bind(ua)
        .bind(country)
        .execute(pool)
    .await;

}

pub async fn get_click_stats(pool: &SqlitePool, slug: &str) -> i64 {

    let row = sqlx::query("SELECT COUNT(*) as total FROM clicks WHERE slug = ?")
        .bind(slug)
        .fetch_one(pool)
        .await
        .unwrap();

    row.get::<i64, _>("total")

}

pub async fn get_unique_visitors(pool: &SqlitePool, slug: &str) -> i64 {

    let row = sqlx::query("SELECT COUNT(DISTINCT ip) as unique_visitors FROM clicks WHERE slug = ? AND ip IS NOT NULL")
        .bind(slug)
        .fetch_one(pool)
        .await
        .unwrap();

    row.get::<i64, _>("unique_visitors")

}

pub async fn get_clicks_by_country(pool: &SqlitePool, slug: &str) -> Vec<(String, i64)> {

    let rows = sqlx::query(r#"SELECT country, COUNT(*) as total FROM clicks WHERE slug = ? AND country IS NOT NULL GROUP BY country ORDER BY total DESC"#)
        .bind(slug)
        .fetch_all(pool)
        .await
        .unwrap();

    rows.into_iter()
        .map(|r| (
            r.get::<String, _>("country"),
            r.get::<i64, _>("total"),
        ))
        .collect()

}

pub async fn get_urls_by_user(pool: &SqlitePool, user_id: &str) -> Result<Vec<(String, String, Option<String>)>, sqlx::Error> {
    
    let rows = sqlx::query("SELECT slug, url, expires_at FROM urls WHERE user_id = ? ORDER BY id DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    let mut out = Vec::new();
    for r in rows {
        out.push((
            r.get("slug"),
            r.get("url"),
            r.get::<Option<String>, _>("expires_at"),
        ));
    }

    Ok(out)

}

pub async fn reset_db(pool: &SqlitePool) -> sqlx::Result<()> {

    sqlx::query("DELETE FROM urls").execute(pool).await?;
    //reset contor = 1
    sqlx::query("DELETE FROM sqlite_sequence WHERE name='urls'").execute(pool).await?;

    sqlx::query("DELETE FROM clicks").execute(pool).await?;
    //reset contor = 1
    sqlx::query("DELETE FROM sqlite_sequence WHERE name='clicks'").execute(pool).await?;

    println!("Database reset successfully: all entries removed and IDs reset.");
    Ok(())

}