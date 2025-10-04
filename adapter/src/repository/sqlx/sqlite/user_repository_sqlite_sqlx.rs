use entity::user::user_entity::UserEntity;
use port::user::user_repository_port::{UserError, UserRepositoryPort, user_repository_dto::*};
use sqlx::Pool;

use crate::repository::sqlx_errors_wrapper::{SqlxErrorClass, SqlxErrorWrap};

pub struct SqliteSqlxUserRepository {}

impl Default for SqliteSqlxUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl SqliteSqlxUserRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[allow(unused_variables)]
impl UserRepositoryPort for SqliteSqlxUserRepository {
    type Tx = Pool<sqlx::sqlite::Sqlite>;
    type Err = sqlx::Error;

    async fn create_user(
        &self,
        cmd: create_user::Command,
        tx: Self::Tx,
    ) -> Result<UserEntity, UserError<Self::Err>> {
        sqlx::query_as::<_, UserEntity>(
            "
            INSERT INTO users (
                name
            ) VALUES ($1)
            RETURNING id, name
        ",
        )
        .bind(cmd.name)
        .fetch_one(&tx)
        .await
        .map_err(|e| e.into())
        .map_err(|e: SqlxErrorWrap| match e.cls {
            SqlxErrorClass::UniqueViolationError => {
                UserError::CreateUserUniqueViolationError(e.orig)
            }
            _ => UserError::Unknown(e.orig),
        })
    }
}
