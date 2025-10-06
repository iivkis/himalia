use std::sync::Arc;

use entity::user::user_entity::UserEntity;
use port::{
    exec::exec_port::ExecutorWrapper,
    user::{
        user_error_port::UserError,
        user_repository_port::{UserRepositoryPort, user_repository_dto},
        user_service_port::{UserServicePort, user_service_dto},
    },
};

pub struct UserService<R>
where
    R: UserRepositoryPort + Send + Sync,
{
    repo: Arc<R>,
}

impl<R> UserService<R>
where
    R: UserRepositoryPort + Sync + Send,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

impl<R> UserServicePort<R::Executor, R::Transaction> for UserService<R>
where
    R: UserRepositoryPort + Sync + Send,
{
    type Exec = ExecutorWrapper<R::Executor, R::Transaction>;

    async fn create_user(
        &self,
        cmd: user_service_dto::create_user::Command,
        exec: Self::Exec,
    ) -> Result<UserEntity, UserError> {
        self.repo
            .create_user(
                user_repository_dto::create_user::Command { name: cmd.name },
                exec,
            )
            .await
    }
}
