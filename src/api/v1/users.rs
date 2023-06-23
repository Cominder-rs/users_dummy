use civilization::utils::make_internal;
use sea_orm::prelude::*;
use tonic::{Request, Response, Status};
use users_dummy_errors::UsersDummyErrors;
use users_dummy_proto::{users_dummy_v1_server::UsersDummyV1, User, UserId};
use users_entities::user::Entity as UserEntity;

pub struct UsersDummyV1Api {
    db: DatabaseConnection,
}

impl UsersDummyV1Api {
    pub fn init(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl UsersDummyV1 for UsersDummyV1Api {
    async fn get_user_by_id(&self, req: Request<UserId>) -> Result<Response<User>, Status> {
        let user_id = req.into_inner().id;

        let user = UserEntity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(make_internal)?
            .ok_or(Status::not_found(
                UsersDummyErrors::UserNotFound.to_string(),
            ))?;

        Ok(Response::new(User {
            id: user.id,
            username: user.username,
            phone_number: user.phone_number,
            firstname: user.firstname,
            lastname: user.lastname,
            city: user.city,
        }))
    }
}
