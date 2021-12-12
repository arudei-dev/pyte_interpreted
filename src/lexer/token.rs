use std::fmt;

pub enum TyOperator {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Multiply,
    /// `/`
    Division,
    /// `(`
    LeftParn,
    /// `)`
    RightParn,
}

impl fmt::Debug for TyOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TyOperator::Plus => "`+`",
                TyOperator::Minus => "`-`",
                TyOperator::Multiply => "`*`",
                TyOperator::Division => "`/`",
                TyOperator::LeftParn => "`(`",
                TyOperator::RightParn => "`)`",
            }
        )
    }
}

#[derive(Debug)]
pub enum TyNumber {
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum Token {
    Operator(TyOperator),
    Number(TyNumber),
    String(String),
}
