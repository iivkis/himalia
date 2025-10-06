#[derive(Debug)]
pub enum UserError {
    Unknown(),
    CreateUserUniqueViolationError(),
}
