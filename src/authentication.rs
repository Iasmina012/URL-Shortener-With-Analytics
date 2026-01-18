use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::collections::{HashMap};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FirebaseClaims {

    pub sub: String, 
    pub aud: String,
    pub iss: String,
    pub exp: usize,
    //jsonwebtoken uses aud, iss, exp internally so its not dead code actually fr
}

pub async fn verify_firebase_token(req: &HttpRequest) -> Result<String, HttpResponse> {

    let header = match req.headers().get("authorization").and_then(|h| h.to_str().ok()) {
        Some(h) => h,
        None => return Err(HttpResponse::Unauthorized().body("Missing authorization token.")),
    };

    if !header.starts_with("Bearer ") {
        return Err(HttpResponse::Unauthorized().body("Missing authorization token."));
    }

    let token = header.trim_start_matches("Bearer ");

    let header = decode_header(token)
        .map_err(|_| HttpResponse::Unauthorized().body("Invalid token header."))?;

    let kid = header.kid
        .ok_or_else(|| HttpResponse::Unauthorized().body("Missing kid."))?;

    let keys: HashMap<String, String> =
        reqwest::get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
            .await
            .map_err(|_| HttpResponse::Unauthorized().body("Failed to fetch keys."))?
            .json()
            .await
            .map_err(|_| HttpResponse::Unauthorized().body("Invalid keys."))?;

    let key = keys
        .get(&kid)
        .ok_or_else(|| HttpResponse::Unauthorized().body("Invalid kid."))?;

    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_audience(&["url-shortener-with-analytics"]);
    validation.set_issuer(&["https://securetoken.google.com/url-shortener-with-analytics"]);

    let decoded = decode::<FirebaseClaims>(
        token,
        &DecodingKey::from_rsa_pem(key.as_bytes()).unwrap(),
        &validation,
    )
        .map_err(|_| HttpResponse::Unauthorized().body("Invalid token."))?;

    Ok(decoded.claims.sub)

}