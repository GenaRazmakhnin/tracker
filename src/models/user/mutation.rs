use crate::entity::{user, user::Entity as User};
use sea_orm::*;
use sea_orm::ActiveValue::Set;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: user::Model,
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            password: Set(form_data.password.to_owned()),
            email: Set(form_data.email.to_owned()),
            username: Set(form_data.username.to_owned()),
            id: Set(uuid::Uuid::new_v4()),
            data: ActiveValue::NotSet,
        }
            .save(db)
            .await
    }


}