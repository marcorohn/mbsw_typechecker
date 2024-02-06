##  Draw a comparison between Haskell and Rust
In general, both languages, Haskell and Rust, allow for implementing a typechecker as described with relative ease.
The author however has found Rust to be "looking" somewhat more familiar for someone coming from a C/Java/Javascript background,
especially in the way functions are declared and called.



### Pattern matching

# ====================================

### Syntax
**Haskell** in uses a very concise syntax, which is very prominent for example when declaring
the custom data types:

```haskell
data E = One | Zero | ETrue | EFalse
       | Plus E E | Mult E E
       | Or E E
```

The **Rust** syntax is more explicit in that regard, enums and pattern matching behave about the same
way as Haskells data types, but the deterministic memory system includes the need for 
i.e. Boxing the expressions, caring for ownership of resources also introduces further syntactic
elements into the language.

```rust
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
```
The Box-Types primary usecase is to put the contained data on the heap instead of the stack. This is needed, as the size 
of i.e. a `Plus` may not always be of the same size (could contain two `Ones`, or more nested expressions), and trying
to resolve such an Expression type would result in an infinite recursion.

Another usecase of the Box-Type is handling data with sizes unknown at compile time, for example when a user
would input expressions at runtime.

Furthermore, Haskells syntax may in fact be somewhat unintuitive when coming from other languages, such as C, Java
or Javascript. Rust is more mainstream in this regard.

### Type System
Both languages support type inference, however, **Haskell** takes the lead and supports full
type inference, including functions.

As **Rust** applies the concept of ownership and borrowing, this has effects on the type system as well,
as not only the information how a struct is layed out in memory and how to interpret it
must be taken into account by the compiler, but also whether it is borrowed or not.

The rust compiler makes sure that each resource has exactly one owner at any time, where the owner is 
the one responsible to destroy the resource again.
This destruction happens, whenever the resource goes out of scope and cannot be reached anymore.
If i.e. we were to pass an Expression object to the typecheck function without additional information,
the compiler would want to destroy the object at the end of the function, which is bad, as it may still be used by the caller.

```rust
// Example without borrowing
fn caller() {
    // first call works
    typecheck(expr);
    // expr has been destroyed, but now what?
    typecheck(expr);
}

fn typecheck(expr: Expr) -> Result<Type, String> {
    match expr {
        Expr::One => Ok(IntType),
        // ...
    }
    // Now, expr goes out of scope, and would be destroyed
}
```

By marking a variable as borrowed, the compiler knows to not destroy/free the underlying object/memory.

Haskell does not require such logic, as it is a garbage collected language, and any implementation has to make sure
to find unused memory and free it.


#### Implementing Traits vs Type Classes

Grouping certain methods together with structs in **Rust** is done via so called traits.
In order to get a textual representation of an expression or any other type, some sort of a
to-string method is needed.

First, a trait must be defined. In the "toString"-example, this is already done by the rust standard library
using the `Display`-trait. It describes one method `fmt`.

Now, the `Display`-trait can be implemented for our `Expr`-type:

```rust
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            One => write!(f, "1"),
            // omitted
            Plus(left, right) => write!(f, "({} + {})", left, right),
            // omitted
        }
    }
}
```

If the `Display`-trait was to contain more method declarations, the `impl`-block above would also
need to implement those, in order to satisfy the constraints set by the trait.


**Haskell** provides a similar concept to traits, so called type classes.
They work in a similar way, by declaring one or more methods for some type.

The counterpart to Rusts `Display` trait, the `Show` type class, may look like the following:
```haskell
class Show s where
    show :: s -> String
```

Using the following `instance Show E where` we tell the compiler,
that there is an instance of the type class `Show` for our expression type `E`.

```haskell
instance Show E where
    show One = "1"
    show Zero = "0"
    show (Plus e1 e2) = "(" ++ show e1 ++ " + " ++ show e2 ++ ")"
    -- omitted some paths
```

#### Type requirements
Both languages provide means to easily extend the functionality of datastructures with default implementations,
such as Rusts `derive`-macro, or the `deriving` keyword in Haskell, as used for the expression datatypes:

```rust 
#[derive(Debug, PartialEq)]
pub enum Expr {
    One,
...
```

```haskell
data E = One | Zero | ETrue | EFalse
       | Plus E E | Mult E E
       | Or E E deriving Eq
```

If that is not enough, the developer can overload operators, such as the == equality operator.

**Rust** requires to implement a trait for the operator to be overwritten, in case for the equality operator this looks like the following:

```rust
impl PartialEq for MyDatastructure {
    fn eq(&self, other: &Self) -> bool {
        // do the actual comparison
    }
}
```

```haskell
instance Eq MyDatastructure where
    (MyDatastructure a1 b1) == (MyDatastructure a2 b2) = -- do the actual comparison
```


### Pattern Matching
Both languages behave roughly the same way, the differences are only of syntactical way:

**Rust** makes use of the `match`-keyword, which also can be nested:

```rust
pub fn typecheck(expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::One => Ok(IntType),
            // omitted
            Plus(e1, e2) => match (typecheck(e1)?, typecheck(e2)?) {
                (IntType, IntType) => Ok(IntType),
                _ => Err("Plus expression expects int types on both sides! "
                    .parse()
                    .unwrap()),
            }
```
whereas 
**Haskell** uses the **case**/**of**, and expressions inside a function
(see the following fragment of the haskell typecheck-function)

```haskell
typecheck :: E -> Maybe Type
typecheck Zero = Just TInt
typecheck (Plus e1 e2) =
     case (typecheck e1, typecheck e2) of
       (Just TInt, Just TInt) -> Just TInt
       (_, _) -> Nothing
```
As can be observed, the "top-level" pattern-matching-expression in rust is a single expression inside the function,
whereas in haskell there is a pattern matching on function level, somewhat resembling piecewise functions from math.


### Error Handling
In **Haskell**, the `Maybe` or `Either` types can be used to signal, that an error occurred, transport
the error outside the function and force the developer to handle it.

Rust uses comparable types, such as `Option` and `Result`. In addition, using the `?`-Operator
can be used to pass-through an error to the calling function, if both functions are of the same
return type.
```rust
 pub fn typecheck(expr: &Expr) -> Result<Type, String> {
        match expr {
            ...
            Plus(e1, e2) => {
                let r1 = typecheck(e1)?;
```
In the above case, it may be unnecessary to as well check for further errors,
if `typecheck(e1)` failed, as the expression in its whole is improperly typed anyway.
So the error can easily be bubbled up through the call hierarchy.

### Immutability
**Haskell** is very strict in that regard allowing only immutable variables, while **Rust**
encourages immutability as well making it the default, but allows for mutable variables 
if the developer chooses so.

### Memory Management
**Haskell** takes the burden of memory management away from the developer by using a garbage collector,
whereas **Rust** ensures memory safety by its ownership system, which, as mentioned before, is noticeable in many other parts of
the programming language, and requires some time and experience getting used to.
