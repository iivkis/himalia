#[derive(Clone)]
pub enum ExecWrap<EXEC, TRANC> {
    Executor(EXEC),
    Transaction(TRANC),
}
