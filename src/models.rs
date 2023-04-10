use diesel::prelude::*;

#[derive(serde::Serialize, Selectable,Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}