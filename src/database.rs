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
            url TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    println!("Database initialized successfully");
    pool

}

pub async fn insert_url(pool: &SqlitePool, slug: &str, url: &str) -> sqlx::Result<()> {

    sqlx::query("INSERT INTO urls (slug, url) VALUES (?, ?)")
        .bind(slug)
        .bind(url)
        .execute(pool)
        .await?;
    Ok(())

}

pub async fn get_url(pool: &SqlitePool, slug: &str) -> Option<String> {

    let row = sqlx::query("SELECT url FROM urls WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await
        .ok()?;

    row.map(|r| r.get::<String, _>("url"))

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