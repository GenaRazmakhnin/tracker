use std::error::Error;
use axum::extract::State;
use axum::http::StatusCode;
use self::models::*;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use super::db::establish_connection;
use super::models;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn show_posts(State(pool): State<Pool>) -> Result<Vec<Post>, dyn Error> {
    use self::super::schema::posts::dsl::*;
    let mut conn = pool.get().await.map_err(internal_error)?;
    posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&mut conn)
        .expect("Error loading posts")
}


fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}