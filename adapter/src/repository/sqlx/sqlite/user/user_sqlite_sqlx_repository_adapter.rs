use std::marker::PhantomData;

use entity::prelude::UserEntity;
use port::prelude::{ExecWrap, UserError, UserRepositoryPort, user_repository_dto::create_user};
use sqlx::{Pool, Transaction};

use crate::repository::sqlx::{
    model::user_model::UserModel,
    sqlx_errors_wrapper::{SqlxErrorClass, SqlxErrorWrap},
};

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
        sqlx::query_as::<_, UserModel>(
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
        .map(|k| k.into())
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
        exec: ExecWrap<Self::Executor, Self::Transaction>,
    ) -> Result<UserEntity, UserError> {
        match exec {
            ExecWrap::Executor(ex) => Self::create_user(cmd, &ex).await,
            ExecWrap::Transaction(mut ex) => Self::create_user(cmd, ex.as_mut()).await,
        }
    }
}
