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

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT UNIQUE NOT NULL,
            url TEXT NOT NULL,
            expires_at TEXT
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

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

    println!("Database initialized successfully");
    pool

}

pub async fn insert_url(pool: &SqlitePool, slug: &str, url: &str, expires_at: Option<String>) -> sqlx::Result<()> {
    
    sqlx::query("INSERT INTO urls (slug, url, expires_at) VALUES (?, ?, ?)")
        .bind(slug)
        .bind(url)
        .bind(expires_at)
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
        Ok(Some((url, expires_at)))
    } else {
        Ok(None)
    }

    //returns Ok(Some((url, expires_at_opt))) if found

}

pub async fn reset_db(pool: &SqlitePool) -> sqlx::Result<()> {
    
    sqlx::query("DELETE FROM urls")
        .execute(pool)
        .await?;

    //reset contor = 1
    sqlx::query("DELETE FROM sqlite_sequence WHERE name='urls'")
        .execute(pool)
        .await?;

    println!("Database reset successfully: all entries removed and IDs reset.");
    Ok(())

}