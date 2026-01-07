use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::collections::{HashMap,HashSet};

#[derive(Debug, Deserialize)]
pub struct FirebaseClaims {

    pub user_id: String,
    pub aud: String,
    pub iss: String,
    pub exp: usize,

}

pub async fn verify_firebase_token(req: &HttpRequest) -> Result<String, HttpResponse> {

    let header = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or("");

    if !header.starts_with("Bearer ") {
        return Err(HttpResponse::Unauthorized().body("Missing auth token"));
    }

    let token = header.trim_start_matches("Bearer ");

    //decode header
    let header = decode_header(token).map_err(|_| {HttpResponse::Unauthorized().body("Invalid token header")})?;

    let kid = header.kid.ok_or_else(|| {HttpResponse::Unauthorized().body("Missing kid")})?;

    //fetch firebase public keys
    let keys: HashMap<String, String> = reqwest::get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
        .await
        .map_err(|_| HttpResponse::Unauthorized().body("Failed to fetch keys"))?
        .json()
        .await
        .map_err(|_| HttpResponse::Unauthorized().body("Invalid keys"))?;

    let key = keys.get(&kid).ok_or_else(|| {HttpResponse::Unauthorized().body("Invalid kid")})?;

    //verify token
    let mut validation = Validation::new(Algorithm::RS256);

    //audience
    let mut aud = HashSet::new();
    aud.insert("url-shortener-with-analytics".to_string());
    validation.aud = Some(aud);

    //issuer
    let mut iss = HashSet::new();
    iss.insert("https://securetoken.google.com/url-shortener-with-analytics".to_string());
    validation.iss = Some(iss);

    let decoded = decode::<FirebaseClaims>(
        token,
        &DecodingKey::from_rsa_pem(key.as_bytes()).unwrap(),
        &validation,
    )
    .map_err(|_| HttpResponse::Unauthorized().body("Invalid token"))?;

    Ok(decoded.claims.user_id)

}