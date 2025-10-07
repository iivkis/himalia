use entity::prelude::UserEntity;
use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct UserModel {
    id: i64,
    name: String,
}

impl From<UserModel> for UserEntity {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
