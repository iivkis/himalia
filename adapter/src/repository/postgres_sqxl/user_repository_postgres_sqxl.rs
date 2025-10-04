use entity::user::user_entity::UserEntity;
use port::user::user_repository_port::{UserRepositoryPort, user_repository_dto::create_user};
use sqlx::{Pool, Postgres};

pub struct PostgresSqlxUserRepository {}

// impl UserRepositoryPort for PostgresSqlxUserRepository {
//     type Tx = Pool<Postgres>;

//     async fn create_user(
//         &self,
//         _cmd: create_user::Command,
//         _tx: Self::Tx,
//     ) -> Result<entity::user::user_entity::UserEntity, ()> {
//         let _ = sqlx::query("").fetch_one(&_tx).await;
//         return Ok(UserEntity {
//             id: 1,
//             name: "".to_string(),
//         });
//     }
// }
