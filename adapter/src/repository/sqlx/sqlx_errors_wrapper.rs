pub struct SqlxErrorWrap {
    pub err_class: SqlxErrorClass,
}

impl SqlxErrorWrap {
    fn new(err_class: SqlxErrorClass) -> Self {
        Self { err_class }
    }
}

pub enum SqlxErrorClass {
    Unknown,
    UniqueViolationError,
    ForeignKeyViolationError,
    CheckViolationError,
}

impl From<sqlx::Error> for SqlxErrorWrap {
    fn from(orig_err: sqlx::Error) -> Self {
        let err_class: SqlxErrorClass = match &orig_err {
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

        Self::new(err_class)
    }
}
