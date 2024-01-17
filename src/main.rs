mod expression;
mod typecheck;
mod util;

use crate::expression::expression::Expr::{And, EFalse, ETrue, Mult, One, Or, Plus, Zero};
use crate::expression::expression::Type::{BoolType, IntType};
use crate::expression::expression::{Expr, Type};
use crate::typecheck::execution::typecheck;
use std::fmt::{Debug, Display};

fn main() {
    // Those must be valid
    println!("Valid Expressions:");
    test(&ETrue, &BoolType);
    test(&Or(Box::new(ETrue), Box::new(EFalse)), &BoolType);
    test(&And(Box::new(EFalse), Box::new(EFalse)), &BoolType);

    println!("\nInvalid Expressions:");
    test(
        &And(
            Box::new(And(Box::new(One), Box::new(Zero))),
            Box::new(Mult(Box::new(One), Box::new(One))),
        ),
        &IntType,
    );

    println!("\nExpressions from lecture:");
    let ex2 = Expr::Plus(Box::new(One), Box::new(ETrue)); // should resolve to invalid
    let ex3 = Expr::Or(Box::new(EFalse), Box::new(ETrue)); // should resolve to boolean
    let ex4 = Expr::Or(Box::new(EFalse), Box::new(One)); // should resolve to invalid
    let ex5 = Expr::Or(Box::new(ETrue), Box::new(One)); // should resolve to invalid

    test(&ex2, &BoolType);
    test(&ex3, &BoolType);
    test(&ex4, &BoolType);
    test(&ex5, &BoolType);
}

fn test(expr: &Expr, expected_type: &Type) {
    let result = typecheck(expr);
    match result {
        Err(e) => {
            println!("Type Error when checking '{}': {}", expr, e)
        }
        Ok(_) => {
            println!("Successfully checked '{}': {}", expr, expected_type)
        }
    }
}
