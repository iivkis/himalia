use std::{path::Path, sync::Arc};

use adapter::repository::sqlite_sqlx::user_repository_sqlite_sqlx::SqliteSqlxUserRepository;
use port::user::user_service_port::{UserServicePort, user_service_dto};
use service::user::user_service::UserService;
use sqlx::{Pool, migrate::Migrator, pool::PoolOptions};

trait GetMigrationPath {
    fn get_migartion_path(&self) -> &'static Path;
}

impl GetMigrationPath for Pool<sqlx::Sqlite> {
    fn get_migartion_path(&self) -> &'static Path {
        Path::new("./migrations/sqlite")
    }
}

impl GetMigrationPath for Pool<sqlx::Postgres> {
    fn get_migartion_path(&self) -> &'static Path {
        Path::new("./migrations/postgres")
    }
}

#[tokio::main]
async fn main() {
    let pool: sqlx::Pool<sqlx::Sqlite> = PoolOptions::new()
        .connect("sqlite://db.sqlite")
        .await
        .unwrap();

    let migrator = Migrator::new(pool.get_migartion_path()).await.unwrap();
    migrator.run(&pool).await.unwrap();

    let user_repo = SqliteSqlxUserRepository::new();

    let user_service = UserService::new(Arc::new(user_repo), pool);

    let user = user_service
        .create_user(
            user_service_dto::create_user::Command {
                name: "Alex".to_string(),
            },
            None,
        )
        .await;

    println!("{:?}", user);
}
