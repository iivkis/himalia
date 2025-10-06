use std::marker::PhantomData;

use entity::user::user_entity::UserEntity;
use port::{
    exec::exec_port::ExecutorWrapper,
    user::user_repository_port::{UserError, UserRepositoryPort, user_repository_dto::*},
};
use sqlx::{Pool, Transaction};

use crate::repository::sqlx::sqlx_errors_wrapper::{SqlxErrorClass, SqlxErrorWrap};

pub struct UserSqliteSqlxRepositoryAdapter<'t> {
    phantom: PhantomData<&'t ()>,
}

impl<'t> Default for UserSqliteSqlxRepositoryAdapter<'t> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'t> UserSqliteSqlxRepositoryAdapter<'t> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
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
impl<'t> UserRepositoryPort for UserSqliteSqlxRepositoryAdapter<'t> {
    type Executor = Pool<sqlx::Sqlite>;
    type Transaction = Transaction<'t, sqlx::Sqlite>;

    async fn create_user(
        &self,
        cmd: create_user::Command,
        exec: ExecutorWrapper<Self::Executor, Self::Transaction>,
    ) -> Result<UserEntity, UserError> {
        match exec {
            ExecutorWrapper::Executor(ex) => Self::create_user(cmd, &ex).await,
            ExecutorWrapper::Transaction(mut ex) => Self::create_user(cmd, ex.as_mut()).await,
        }
    }
}
