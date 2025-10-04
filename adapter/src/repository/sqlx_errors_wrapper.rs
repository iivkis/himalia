pub struct SqlxErrorWrap {
    pub orig: sqlx::Error,
    pub cls: SqlxErrorClass,
}

impl SqlxErrorWrap {
    fn new(orig: sqlx::Error, cls: SqlxErrorClass) -> Self {
        Self { orig, cls }
    }
}

pub enum SqlxErrorClass {
    Unknown,
    UniqueViolationError,
    ForeignKeyViolationError,
    CheckViolationError,
}

impl From<sqlx::Error> for SqlxErrorWrap {
    fn from(err: sqlx::Error) -> Self {
        let cls: SqlxErrorClass = match &err {
            sqlx::Error::Database(dberr) => {
                if dberr.is_unique_violation() {
                    SqlxErrorClass::UniqueViolationError
                } else if dberr.is_foreign_key_violation() {
                    SqlxErrorClass::ForeignKeyViolationError
                } else if dberr.is_check_violation() {
                    SqlxErrorClass::CheckViolationError
                } else {
                    SqlxErrorClass::Unknown
                }
            }
            _ => SqlxErrorClass::Unknown,
        };

        Self::new(err, cls)
    }
}
