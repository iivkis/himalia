use adapter::prelude::UserSqliteSqlxRepositoryAdapter;
use clap::Parser;
use port::{
    exec::exec_port::ExecutorWrapper,
    user::user_service_port::{UserServicePort, user_service_dto},
};
use service::user::user_service::UserService;
use sqlx::{Pool, migrate::Migrator, pool::PoolOptions};
use std::{path::Path, sync::Arc};

mod cmd_flags;

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
    let cmd_flags = cmd_flags::CmdFlags::parse();

    let pool = PoolOptions::new()
        .max_connections(10)
        .test_before_acquire(true)
        .connect("sqlite://data/db.sqlite")
        .await?;

    if cmd_flags.only_migration {
        println!("run migrator..");
        Migrator::new(pool.get_migartion_path())
            .await
            .unwrap()
            .run(&pool)
            .await?;
        println!("migration success!");
        return Ok(());
    } else {
        println!("run without migration");
    }

    let user_repo = UserSqliteSqlxRepositoryAdapter::new();

    let user_service = Arc::new(UserService::new(Arc::new(user_repo)));

    println!("start!");

    let mut tasks = vec![];
    for _ in 0..100_000 {
        let user_service_cloned = user_service.clone();
        let pool_cloned = pool.clone();

        let task = tokio::spawn(async move {
            let _user = user_service_cloned
                .create_user(
                    user_service_dto::create_user::Command {
                        name: "Alex".to_string(),
                    },
                    ExecutorWrapper::Executor(pool_cloned),
                )
                .await
                .map_err(|err| println!("{err:?}"));
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
