pub mod execution {
    use crate::expression::expression::Expr;
    use crate::expression::expression::Expr::{And, Mult, Or, Plus};
    use crate::Type;
    use crate::Type::{BoolType, IntType};
    use crate::util::util::Either;
    use crate::util::util::Either::{Left, Right};

    pub fn typecheck(expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::One => Ok(IntType),
            Expr::Zero => Ok(IntType),
            Expr::ETrue => Ok(BoolType),
            Expr::EFalse => Ok(BoolType),
            Plus(e1, e2) => {
                let r1 = typecheck(e1)?;
                let r2 = typecheck(e2)?;
                match (r1, r2) {
                    (IntType, IntType) => Ok(IntType),
                    _ => Err("Plus expression expects int types on both sides! "
                        .parse()
                        .unwrap()),
                }
            }
            Mult(e1, e2) => {
                let r1 = typecheck(e1)?;
                let r2 = typecheck(e2)?;
                match (r1, r2) {
                    (IntType, IntType) => Ok(IntType),
                    _ => Err("Mult expression expects int types on both sides! "
                        .parse()
                        .unwrap()),
                }
            }
            Or(e1, e2) => {
                let r1 = typecheck(e1)?;
                let r2 = typecheck(e2)?;
                match (r1, r2) {
                    (BoolType, BoolType) => Ok(BoolType),
                    _ => Err("Mult expression expects bool types on both sides! "
                        .parse()
                        .unwrap()),
                }
            }
            And(e1, e2) => {
                let r1 = typecheck(e1)?;
                let r2 = typecheck(e2)?;
                match (r1, r2) {
                    (BoolType, BoolType) => Ok(BoolType),
                    _ => Err("Mult expression expects bool types on both sides! "
                        .parse()
                        .unwrap()),
                }
            }
        }
    }


    pub fn eval_t(expr: &Expr) -> Result<Either<i32, bool>, String> {
        match expr {
            Expr::One => Ok(Left(1)),
            Expr::Zero => Ok(Left(0)),
            Expr::ETrue => Ok(Right(true)),
            Expr::EFalse => Ok(Right(false)),
            Expr::Plus(e1, e2) => {
                let r1 = eval_t(e1)?;
                let r2 = eval_t(e2)?;
                match (r1, r2) {
                    (Left(a), Left(b)) => Ok(Left(a + b)),
                    _ => Err("Incompatible Types!".parse().unwrap()),
                }
            }
            Expr::Mult(e1, e2) => {
                let r1 = eval_t(e1)?;
                let r2 = eval_t(e1)?;
                match (r1, r2) {
                    (Left(a), Left(b)) => Ok(Left(a * b)),
                    _ => Err("Incompatible Types!".parse().unwrap()),
                }
            }
            Expr::Or(e1, e2) => {
                let r1 = eval_t(e1)?;
                let r2 = eval_t(e2)?;
                match (r1, r2) {
                    (Right(a), Right(b)) => Ok(Right(a || b)),
                    _ => Err("Incompatible Types!".parse().unwrap()),
                }
            }

            Expr::And(e1, e2) => {
                let r1 = eval_t(e1)?;
                let r2 = eval_t(e2)?;
                match (r1, r2) {
                    (Right(a), Right(b)) => Ok(Right(a && b)),
                    _ => Err("Incompatible Types!".parse().unwrap()),
                }
            }
        }
    }
}


