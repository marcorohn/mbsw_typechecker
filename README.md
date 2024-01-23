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

Furthermore, Haskells syntax may in fact be somewhat unintuitive when coming from other languages, such as C, Java
or Javascript. Rust is more mainstream in this regard.

### Type System
Both languages support type inference, however, **Haskell** takes the lead and supports full
type inference, including functions.

As **Rust** applies the concept of ownership and borrowing, this has effects on the type system as well,
as not only the information how a struct is layed out in memory and how to interpret it
must be taken into account by the compiler, but also whether it is borrowed or not.


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

#### Implementing Rraits vs Type Classes
Of course, it is not possible to implement all sorts of functionality this way,
but we might still want to tell the compiler, that certain types are usable with certain functions.
One such usecase is the ability to get a string representation of an object:

**In Rust,** this is done by implementing a trait for a given type.

**In Haskell however,** things are again a bit more concise:

```haskell
instance Show E where
    show One = "1"
    show Zero = "0"
    ...
```

Using `instance Show E where` we tell the compiler, that there is an instance of the type class `Show` for our expression type `E`.
It compares to the 

### Pattern Matching
Both languages behave roughly the same way, the differences are only of syntactical way:

**Rust** makes use of the `match`-keyword, which also can be nested, whereas 
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
