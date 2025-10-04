pub use crate::user::user_error_port::UserError;
use entity::user::user_entity::UserEntity;
use user_repository_dto::*;

pub mod user_repository_dto {
    pub mod create_user {
        pub struct Command {
            pub name: String,
        }
    }
}

pub trait UserRepositoryPort: Sync + Send {
    type Tx;
    type Err;

    fn create_user(
        &self,
        cmd: create_user::Command,
        tx: Self::Tx,
    ) -> impl Future<Output = Result<UserEntity, UserError<Self::Err>>> + Send;
}
