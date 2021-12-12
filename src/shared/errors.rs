#[derive(Debug)]
pub struct ErrorDetail {
    pub pos_start: usize,
    pub pos_end: usize,
    pub details: String,
}

#[derive(Debug)]
pub enum CompileError {
    IllegalChar(ErrorDetail),
    InvalidSyntax(ErrorDetail),
}
