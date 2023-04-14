use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::{Error, Result, web::AUTH_TOKEN};


pub fn routes() -> Router{
    Router::new().route("/auth/login", post(api_login))
}


async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>>{
    if payload.username != "tracker" || payload.password != "tracker"{
        return Err(Error::LoginFail)
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));
    let body = Json(json!({"result": {"success": true}}));
    Ok(body)
}



#[derive(Debug,Deserialize)]
struct LoginPayload{
    username: String,
    password: String,

}