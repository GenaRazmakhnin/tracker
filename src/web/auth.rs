use std::fmt::Display;
use axum::{async_trait, Form, Json, RequestPartsExt, Router, TypedHeader};
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get,post};
use axum_csrf::CsrfToken;
use headers::{Authorization, HeaderMapExt};
use headers::authorization::Bearer;
use http::request::Parts;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use tower_cookies::Cookies;
use crate::{AppState, Error, Result};


pub fn routes() -> Router<AppState> {

    Router::new()
        .route("/auth/authorize", post(authorize))
        .route("/auth/login", get(login_page).post(login))
}

async fn login_page(token: CsrfToken, State(state): State<AppState>) -> impl IntoResponse {

    let mut ctx = tera::Context::new();
    ctx.insert("csrf_token", &token.authenticity_token());

    let body = state
        .templates
        .render("auth/login.html.tera", &ctx)
        .map_err(|_| Error::TemplateError).expect("Template Render Error");

    (token, Html(body)).into_response()
}

#[derive(Clone,Deserialize)]
struct LoginPayload{
    csrf_token: String,
    username: String,
    password: String,
}

async fn login(token: CsrfToken, form_data: Form<LoginPayload>) -> Result<Html<String>>{
    if token.verify(&form_data.clone().csrf_token).is_err() {
        Ok(Html("Token is invalid".to_string()))
    } else {
        Ok(Html("Ok".to_string()))
    }
}




static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});


pub async fn mw_require_auth<B>(
    req: Request<B>,
    next: Next<B>)
    -> Result<Response> {
    let bearer = req.headers().typed_get::<Authorization<Bearer>>().ok_or(Error::InvalidToken)?;
    let _ = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
        .map_err(|_| Error::InvalidToken)?;

    Ok(next.run(req).await)
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(Error::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(Error::WrongCredentials);
    }
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| Error::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}


impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}


#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> core::result::Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| Error::InvalidToken)?;

        Ok(token_data.claims)
    }
}


struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}


