#[derive(Debug)]
pub enum UserError<E> {
    Unknown(E),
    CreateUserUniqueViolationError(E),
}
