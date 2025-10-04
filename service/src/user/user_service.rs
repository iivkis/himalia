use std::sync::Arc;

use entity::user::user_entity::UserEntity;
use port::user::{
    user_error_port::UserError,
    user_repository_port::{UserRepositoryPort, user_repository_dto},
    user_service_port::{UserServicePort, user_service_dto},
};

pub struct UserService<R>
where
    R: UserRepositoryPort + Send + Sync,
    R::Tx: Send + Sync + Clone,
{
    repo: Arc<R>,
    tx: R::Tx,
}

impl<R> UserService<R>
where
    R: UserRepositoryPort + Sync + Send,
    R::Tx: Send + Sync + Clone,
{
    pub fn new(repo: Arc<R>, tx: R::Tx) -> Self {
        Self { repo, tx }
    }
}

impl<R> UserServicePort<R::Tx, R::Err> for UserService<R>
where
    R: UserRepositoryPort + Sync + Send,
    R::Tx: Sync + Send + Clone,
    R::Err: Sync + Send,
{
    async fn create_user(
        &self,
        cmd: user_service_dto::create_user::Command,
        tx: Option<R::Tx>,
    ) -> Result<UserEntity, UserError<R::Err>> {
        self.repo
            .create_user(
                user_repository_dto::create_user::Command { name: cmd.name },
                tx.unwrap_or(self.tx.clone()),
            )
            .await
    }
}
