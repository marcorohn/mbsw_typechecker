pub mod expression {
    use crate::expression;
    use crate::expression::expression::Expr::{And, EFalse, ETrue, Mult, One, Or, Plus, Zero};
    use std::fmt::{Display, Formatter};

    #[derive(Debug, PartialEq)]
    pub enum Expr {
        One,
        Zero,
        ETrue,
        EFalse,
        Plus(Box<Expr>, Box<Expr>),
        Mult(Box<Expr>, Box<Expr>),
        Or(Box<Expr>, Box<Expr>),
        And(Box<Expr>, Box<Expr>),
    }
    impl Display for Expr {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                One => write!(f, "1"),
                Zero => write!(f, "0"),
                ETrue => write!(f, "true"),
                EFalse => write!(f, "false"),
                Plus(left, right) => write!(f, "({} + {})", left, right),
                Mult(left, right) => write!(f, "({} * {})", left, right),
                Or(left, right) => write!(f, "({} || {})", left, right),
                And(left, right) => write!(f, "({} && {})", left, right),
            }
        }
    }

    /*
     * Represents the type of an expression.
     */
    pub enum Type {
        IntType,
        BoolType,
    }

    impl Display for Type {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Type::IntType => {
                    write!(f, "IntType")
                }
                Type::BoolType => {
                    write!(f, "BoolType")
                }
            }
        }
    }
}
