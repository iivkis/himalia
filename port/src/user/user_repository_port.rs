use crate::exec::exec_port::ExecWrap;
pub use crate::user::user_error_port::UserError;
use entity::prelude::UserEntity;
use user_repository_dto::*;

pub mod user_repository_dto {
    pub mod create_user {
        pub struct Command {
            pub name: String,
        }
    }
}

pub trait UserRepositoryPort: Sync + Send {
    type Executor: Send + Sync;
    type Transaction: Send + Sync;

    fn create_user(
        &self,
        cmd: create_user::Command,
        exec: ExecWrap<Self::Executor, Self::Transaction>,
    ) -> impl Future<Output = Result<UserEntity, UserError>> + Send;
}
