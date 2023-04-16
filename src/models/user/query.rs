use crate::entity::{user, user::Entity as User};
use sea_orm::*;
use uuid::Uuid;

pub struct UserQuery;

impl UserQuery {
    pub async fn find_post_by_id(db: &DbConn, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }
}