use entity::user::user_entity::UserEntity;
use port::user::user_repository_port::{UserError, UserRepositoryPort, user_repository_dto::*};
use sqlx::Pool;

pub struct SqliteSqlxUserRepository {}

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
        .map_err(|err| {
            if let sqlx::Error::Database(dberr) = &err {
                if dberr.is_unique_violation() {
                    return UserError::CreateUserUniqueViolationError(err);
                }
            }
            UserError::Unknown(err)
        })
    }
}
