use entity::user::user_entity::UserEntity;
use user_service_dto::*;

use crate::user::user_error_port::UserError;

pub mod user_service_dto {
    pub mod create_user {
        pub struct Command {
            pub name: String,
        }
    }
}

pub trait UserServicePort<EXEC, TRANC>: Send + Sync {
    type Exec: Send + Sync;

    fn create_user(
        &self,
        cmd: create_user::Command,
        exec: Self::Exec,
    ) -> impl Future<Output = Result<UserEntity, UserError>> + Send;
}
