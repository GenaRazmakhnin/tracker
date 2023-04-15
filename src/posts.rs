use diesel::prelude::*;
use axum::http::StatusCode;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use crate::Pool;


#[derive(serde::Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}



pub async fn show_posts(pool: Pool) -> Result<Vec<Post>, (StatusCode, String)> {
    use crate::schema::posts::dsl::*;
    let mut conn = pool.get().await.map_err(internal_error)?;
    posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&mut conn)
        .await
        .map_err(internal_error)
}


fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}