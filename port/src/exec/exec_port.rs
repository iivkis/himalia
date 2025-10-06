#[derive(Clone)]
pub enum ExecutorWrapper<EXEC, TRANC> {
    Executor(EXEC),
    Transaction(TRANC),
}
