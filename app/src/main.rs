use std::{path::Path, sync::Arc};

use adapter::repository::sqlx::UserSqliteSqlxRepositoryAdapter;
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool: sqlx::Pool<sqlx::Sqlite> = PoolOptions::new()
        .max_connections(10)
        .test_before_acquire(true)
        .connect("sqlite://db.sqlite")
        .await?;

    let migrator = Migrator::new(pool.get_migartion_path()).await.unwrap();
    migrator.run(&pool).await?;

    let user_repo = UserSqliteSqlxRepositoryAdapter::new();

    let user_service = Arc::new(UserService::new(Arc::new(user_repo), pool));

    println!("start!");

    let mut tasks = vec![];
    for _ in 0..100_000 {
        let user_service_cloned = user_service.clone();

        let task = tokio::spawn(async move {
            let user = user_service_cloned
                .create_user(
                    user_service_dto::create_user::Command {
                        name: "Alex".to_string(),
                    },
                    None,
                )
                .await
                .map_err(|err| println!("{:?}", err));
        });

        tasks.push(task);
    }

    println!("done!");

    for t in tasks {
        t.await?;
    }

    println!("finish!");

    Ok(())
}
