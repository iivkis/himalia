use entity::user::user_entity::UserEntity;
use port::user::user_repository_port::{
    ExecutorWrapper, UserError, UserRepositoryPort, user_repository_dto::*,
};
use sqlx::{Pool, Transaction};

use crate::repository::sqlx::sqlx_errors_wrapper::{SqlxErrorClass, SqlxErrorWrap};

pub struct UserSqliteSqlxRepositoryAdapter {}

impl Default for UserSqliteSqlxRepositoryAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl UserSqliteSqlxRepositoryAdapter {
    pub fn new() -> Self {
        Self {}
    }

    async fn create_user<'ex, EX>(
        cmd: create_user::Command,
        exec: EX,
    ) -> Result<UserEntity, UserError>
    where
        EX: sqlx::Executor<'ex, Database = sqlx::Sqlite>,
    {
        sqlx::query_as::<_, UserEntity>(
            "
            INSERT INTO users (
                name
            ) VALUES ($1)
            RETURNING id, name
        ",
        )
        .bind(cmd.name)
        .fetch_one(exec)
        .await
        .map_err(|e| e.into())
        .map_err(|e: SqlxErrorWrap| match e.err_class {
            SqlxErrorClass::UniqueViolationError => UserError::CreateUserUniqueViolationError(),
            _ => UserError::Unknown(),
        })
    }
}

#[allow(unused_variables)]
impl UserRepositoryPort for UserSqliteSqlxRepositoryAdapter {
    type Executor = Pool<sqlx::Sqlite>;
    type Transaction = Transaction<'static, sqlx::Sqlite>;

    async fn create_user(
        &self,
        cmd: create_user::Command,
        tx: ExecutorWrapper<Self::Executor, Self::Transaction>,
    ) -> Result<UserEntity, UserError> {
        match tx {
            ExecutorWrapper::Executor(ex) => Self::create_user(cmd, &ex).await,
            ExecutorWrapper::Transaction(mut ex) => Self::create_user(cmd, ex.as_mut()).await,
        }
    }
}
