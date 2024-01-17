pub mod util {
    use std::fmt::{Debug, Display, Formatter};
    use crate::util::util::Either::{Left, Right};


    #[derive(Debug, PartialEq)]
    pub enum Either<A, B> {
        Left(A),
        Right(B),
    }

    // Make the result nicer to look at
    impl<A: Display, B: Display> Display for Either<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Left(a) => write!(f, "{}", a),
                Right(b) => write!(f, "{}", b),
            }
        }
    }
}