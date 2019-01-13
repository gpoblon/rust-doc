# RUST

# Side Notes
- Statements vs expressions: statements perform an action but do not return a value. Expr evaluate to a resulting value. But expressions can be part of statements (ex: `6` expr from `let = 6;` statement).

# Code Conventions
- variables are snake case (lowercase + underscore)
- variable signature is optional
- function parameters signature is mandatory
- function return signature is optional
- use the `use` keyword to bring parent module of an elem (fun, struct, enum...) into scope, not directly the element
- exception: if two types have the same name, use `use` on its parent or rename it with `use std::io::Result as IoResult;`

# INSTALLATION (1.1)

On macOS: `curl https://sh.rustup.rs -sSf | sh`
On Windows: `https://www.rust-lang.org/install.html`
Main commands:
`rustup update`
`rustc --version`; `rustc x.y.z` to hotswap versions.
`rustup doc` offline doc
`rustc main.rs && ./main` to compile then execute by hand (`./main.exe` on Windows).
Cargo (automatically installed with rustup):
`cargo --version`
`cargo new pjname` creates a project with a Cargo.toml file + basic main.
`cargo build` compiles and creates an exe in `./target/debug/pjname` + creates a Cargo.lock the first time to keep a track of dependencies versions used.
`cargo build --release` compiles with optimizations and creates exe in `./target/release/pjname`
`cargo check` checks code but does not actually creates the exe. Faster.
`cargo run` compiles + runs it (== `cargo build && ./target/debug/pjname`).
NB: 2 kinds of errors when compiling. Compilation errors and runtime errors are checked by the `cargo run` command.

# MAIN CONCEPTS (3)

- Keywords:
    Keywords names are reserved unless a "raw identifier" is used: `r#keyword`
- Variables
    `let`. By default immutable unless `mut` is used. Limited to their scope.
    Different than `const MAX_POINTS: u32 = 100_000;` which are global and can only be set to constant expr (!= fun call / runtime computed variables).
    Shadowing (`let x = 5; let x = x + 5;`) is another way to reassign. Differences: redeclared every change + copy (vs ref) + type can change.

## Data types (3.2)
Scalar types:
- integers:
    i8 / 16 / 32 / 64 / 128 / isize or u8...
    Size is the archi depending size ex: 64bit achi = i64.
    Int literals: decimal - hexa (`0x`) - octa (`0o`) - binary(`0b`) - byte (u8 only)
    `_` is a separator.
    NB: u8 = 0 - 255 ; default Int is i32 ; in debug mode overflow is tested and leads to 'panic'. But in release mode: i8 -> 256 becomes 0.
- floats:
    f32 / 64. Default is f64 = double.
- bools:
    one byte size.
- chars:
    whatever unicode size. `'Z'` or `U+0000` or even smileys.

Compound types:
- Tuples:
    groups of several values that can have different types.
    `let tup: (i32, f64, u8) = (500, 6.4, 1);`.
    Can be destructured: `let (x, y, z) = tup;`.
    Access to elements : `tup.1`
- Arrays:
    type must be the same and fixed length.
    Arrays are on the stack.
    `let arr: [i32; 5] = [1, 2, 3, 4, 5];`
    Access to elements: `arr[0]`.
    NB: out of bound access is checked by the compiler.

## Functions (pervasives) (3.3)
function return = NO SEMICOLON.
Several values can be returned via tuples.
```
fn main() {
    let (x, y) = get_new_xy(5, 6);
}

fn get_new_xy(x: i32, y: i32) -> i32 {
    println!("The value of x is {}", x); // calls a macro (`!`) not a function.
    (x + 5, y + 2)
}
```
Rust does not care where functions are defined.

## Control Flow (3.5)
- `if cond {}` Expr: condition must be a bool. CExemple of possibility:
```
let number = if condition {
        5
    } else {
        6 // ! must return the same type or ERR.
    };
```
- `loop {}` = forever until `break`
- `while cond {}`
- `for number in (1..4) {}` = safest = more commonly used

# OWNERSHIP (4)

## Definition (4.1)
Central and unique Rust feature, ownership exists to manage heap data. No need for garbage collector.
Rules:
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
- Value and variable are distinct as the variable can be rewritten after its value as been freed.

Ex through String.
`let s = "hello";` is a string literal (hardcoded) vs `let mut s1 = String::from("hello");` that can be edited and is located on the heap.
String is composed of 3 variables in the stack: a ptr to the heap ; a len ; a capacity (nbr of allocated bytes from the system).
-  Move copy: `let s2 = s1;` s1 is moved into s2 AND `s1` value is freed and becomes invalid. `s1` is still available as a variable and can therefore be rewritten. s2 is a copy of the stack but still, is a reference.
- Deep clone: `let s2 = s1.clone();` copies the whole data, creating a new pointer.

Some types have the `Copy` trait such as: integer, bool, float, char. Tuple too if they contain only `Copy` types. Means that after `let x = 5; let y = x;`, `x` is still valid.

Ownership can be transmitted: by returning the variable or passing it to a function as an argument. So be careful when passing a heap variable to a function: return it. So tedious. Solution is to use references

## References (borrowing) (4.2)
Allows to pass values without passing ownership.
```
let len = get_len(&s1);

fn get_len(s: &String) -> usize {
    s.len
}
```
`s` contains only the ptr to s1.

##### Mutable references
A borrowed value cannot be modified unless `mut` is used: `...fn get_len(s: &String) -> usize {...`. Ofc the initial `String` has to be `mut` too.
Restriction: `let r1 = &mut s; let r2 = &mut s;` only one mut ref of a data at a given scope. Trick is to create a new scope (`{ let r1 = &mut s; }`)
Useful because if the ownership is passed the variable dies in the new scope and becomes impossible to use in the previous one.
Can have multiple immutable ref but not a mut while having an immutable one. Security measure, ex:
```
let first = &v[0]; // immutable borrow
v.push(6); // mutable borrow
```
will not compile because if the new element pushed implies a reallocation, the reference to the first would then point to a deallocated zone = crash.


## Dangling references
Cannot create a variable in a scope and return a reference to this variable. You must return the variable directly.

## Slices (4.3)
It is possible to keep a reference of a portion of a String: (all following statements are equal): 
```
let s = String::from("hello");
let len = s.len();

let hello = &s[0..5];
let hello = &s[0..=4];
let hello = &s[..];
let hello = &s[..len];
```
A String can be passed as a literal: `&s[..]` which has a `&str` type rather than `&String` type. Doing this also works with literal so it is more generic.
Of course slice works with every type of collections, not just `String`.


# STRUCTURES (5)

## 5.1 Defining and instanciating structs
Like tuples but each variable is named. 
```
Struct User { // definition
    username: String,
    email: String
}

let mut user1 = User { // initialisation, not necessarily mutable
    email: String::from("test@gmail.com"),
    username: String::from("test")
};

user1.email = String::from("updatedemail@gmail.com");

fn build_user(email: String, username: String) -> User {
    User { email, username } // shorthand if param and field name are the same
}
```
Note that either the entire struct is mutable or it is not at all, fields cannot.

##### Creating instances from other instances with stuct update syntax
`let user2 = User { ..user1 };`

##### Tuple structs
structs without named fields: `struct Color(i32, i32, i32); let black = Color(0, 0, 0);` vs tuple `let color = (0, 0, 0);`
The only difference with tuples is that the struct is named and cannot be swapped with an equivalent typed struct as a function param for exemple.

##### Unit-like structs
Empty `()` structs. Useful when... see later.

##### Ownership of struct data
Structs can have references owned by something else using the lifetimes feature.

## Debug a struct (5.2)
```
#[derive(Debug)] // needed to println
struct Rectangle { width: u32, height: u32 }

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1); // `:?` identifier needed too or `:#?` for another output format
}
```

## Method Syntax (5.3)
Methods are fun defined in the context of a struct, their first param is always `&self` (the instance of the struct).
```
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // (self) is sugar for (self: &Self)
        self.width * self.height
    }
}
```
and is called by: `rect1.area();`.
`self` is only borrowed because it just reads, does not write (`&mut self`) / does not need ownership (rare).
Rust has a feature called 'automatic de/referencing', adding `& mut *` to match the object signature so `->` is not needed to access an method.

##### Associated functions
functions != methods: are part of the struct BUT they do not have a `self` param. Often use to create a new instance of the struct.
It is called by `rect1::my_fun();`.

##### Multiple `impl` Blocks
Sometimes useful.

# ENUMS AND PATTERN MATCHING (6)
## Definition of Enums (6.1)
```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
accessed by: `let changeColor = Message::ChangeColor(0, 255, 0);`.
Can be used as struct field or a param: `fn route(message: Message) { }` then called `route(Message::Quit);`.
An enum can contain data (including methods, structs, enums).
Methods are declared as they are in structs, using `impl` and are called on any elem of the enum: `Message::Write(String::from("hello")).call();`

##### Option type (enum)
Replaces the `null` value that cause too many errors.
```
enum Option<T> {
    Some(T),
    None,
}

let some_string = Some(5);
let absent_number: Option<i32> = None;
```
Option type must be specified if `None` is used bc it cannot be inferred.
Be careful! This does not work:
```
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y; // adding an int to an option -> compilation error.
```

## The `match` control flow operator (6.2)
Code can be executed in a `match` expr, receive params and return an expr:
```
enum Coin {
    Penny,
    Nickel,
    Quarter(UsState : String), 
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State quarter from {}", state);
            25
        },
    }
}
```
`match` is commonly used against `enum`s.
##### Matching with `Option<T>`
```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```
##### Match is exhaustive
...so all cases must be handled (if not: compilation error).
##### The placeholder `_`
matches any value
```
    match u8_value {
        1 => 1
        5 => 5,
        _ => (), // if not 1 or 5 do nothing (unit)
    }
```

## Concise control flow with `if let` (6.3)
`match` one pattern while ignoring the rest. Useful when only one particular value is interesting.
```
if let Some(3) = some_u8_val {
    println!("three");
} else {
    println!("None");
}

// is the same thing that
match some_u8_val {
    Some(3) => println!("three"),
    _ => println!("None"),
}
```
Can have a `else` expr.

# PACKAGES, CRATES AND MODULES (7)
Rust features related to scope.
- A package is a Cargo feature that let you build, test, and share crates. (like nodejs packages)
- A crate is a tree of modules that produce a binry / library or executable.
- A module and the `use` keyword let you control the scope and privacy of paths.
- A path is a way of naming an item such as a struct, function, or module.

## Packages and crates for making libraries and executables (7.1)
Crate = binary / library.
A crate has a root, a source file that defines how to build the crate.
A package has a Cargo.toml file that describes how to build one+ crates.
`cargo new` creates a package (bc it has a Cargo.toml file).
Convention: Rust knows when there is a binary crate located at *src/main.rs* or a library crate located at *src/lib.rs*
A package can have 0 / 1 binary crate but +inf libs. But if it has a lib and a binary it has 2 crates with the same names.
Multiple binaries is possible by placing files in *src/bin/*, each file will be a separate binary crate.

## The module system to control scope and privacy (7.2)
create a module with crate roots:
```
mod sound {
    mod instrument {
        fn guitar() {}
    }
    mod voice {}
}
```
#### Paths
- absolute: `crate::sound::voice`
- relative: `sound::voice` and uses `self` `super` or an identifier in the current module.
#### Modules as the Privacy Boundary
`pub mod` makes current `mod` public but its components stay private.
ALL children `mod` elements are private by default. But current and ancestor `mod` are public for the current module.
```
mod sound {
    pub mod instrument {
        pub fn clarinet() {}
    ...
```
if `clarinet()` was not pub it would not be accessible from the root of the crate.

##### Starting Relative Paths with `super`
`super` is equivalent to: `../` to the path: calling `super::foo()` calls the parents' module `foo` function. Ex:
```
mod sound {
    mod instrument {
        fn clarinet() {
            super::breathe_in(); // context is now sound
        }
    }

    fn breathe_in() {} // child of sound module
}
```
##### Using `pub` with structs and enums
If `pub` is used before `struct` its fields are still private, even simple variables.
If an `enum` is public its elements are too.
##### The `use` keyword to bring paths into a scope
`use` can be called from everywhere (function, root, module etc).
`use crate::sound::instrument;` brings an absolute path into scope and allows to directly use `instrument::whatever`.
To bring a relative path into scope (ie from the current scope), use the `self` keyword: `use self::sound::instrument;`.
##### Idiomatic `use` paths for functions vs. other items
`use` path for functions / structs / enums etc is a bad practice (`use crate::sound::instrument::clarinet;`). It is better to use `use` on modules to keep it clear that path definition is not local.
##### Renaming types brought into scope with the `as` keyword
if two types have the same name, either: bring its parent to scope (exception of the previous rule) or rename it: `use std::io::Result as IoResult;`
##### Re-exporting names with `pub use`
```
mod performance_group {
    pub use crate::sound::instrument;
}
```
`instrument` becomes available not only for the `performance_group` module but for others to bring it into their scope.
##### Using external packages
*Cargo.toml file*
```
[dependencies]
rand = "0.5.5"
```
makes rand available from everywhere but `use rand::needed_trait` is needed to bring a trait of a package into scope.
Exception: `std` (which is an absolute path) is shipped with rust so no need to update *Cargo.toml* but `use` is still requiered to bring items into scope.
##### Nested paths forcleaning up large `use` lists
`use std::io::{self, Write};` brings into scope the `std::io` and `std::io::Write` modules.
##### Bringing all public definitions into scope (Glob operator)
`use std::collections::*;` to use sparingly (mainly `tests` module)

#### Separating modules into different files
sound could be moved into a *src/sound.rs* file. It would be called in main as:
```
mod sound; // semicolon tells Rust to load a module with the same name as the module.

fn main() {
    crate::sound::instrument::clarinet();
    // OR
    sound::instrument::clarinet();
}
```
The `instrument` module can be isolated too: *src/sound/instrument.rs* can be created to handle the `instrument` module and leave only the `mod instrument;` into the `sound` module.

# COMMON COLLECTIONS (8)
Stored on the heap vs tuples and build-in arrays.
Amount of data stored does not need to be known at compile time.
Most commonly used: Vectors, Strings and Hash Maps.

## Vectors `Vec<T>` (8.1)
Store several values of any single data type.
##### Create
`let v: Vec<i32> = Vec::new();` but if there are initial values type can be infer using `vec!` macro: `let v - vec![1, 2, 3];`
##### Update
```
let mut v = vec![1, 2, 3];
v.push(5);
```
There is a pop method too that removes and returns the last elements.
##### Drop
The vector and all its elements are dropped when out of scope.
##### Reading elements
`&v[9]` or `v.get(9)` gets the ninth elem. If the index does not exist:
- `[]` will panic. Use it if non-existant indexes should not be passed.
- with the accessor `get` it will return `None`. Should be used with the `match` expr: 
    ```
    match v.get(9) {
        Some() => println("Exists!"),
        None => println("Does not exist."),
    }
    ```
    Use it if it is normal to hit non-existant indexes.

Impossible to hold an immutable ref to a vector and try to add elems: 
```
let first = &v[0]; // immutable borrow
v.push(6); // mutable borrow
```
will not compile because if the new element pushed implies a reallocation, the reference to the first would then point to a deallocated zone = crash.
##### Iterate
read: `for i in &v {}`
write: `for i in &mut v { *i += 50; }`. Note that i has to be dereferenced before it can be used with the `+=` operator.
##### Using Enums to store multiple types
```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
Nice trick to use any exhaustive types. 

## Strings (8.2)
Collection of UTF-8 encoded chars.
##### Definition
By opposition to *string literals* `str` which are stored in the binary output of the program, `String`s are stored in the heap.
##### Creation
`let mut s = String::new();` creates an empty string
`let s = "test".to_string();` OR `let s = String::from("test");` create a `String` from a literal
##### Update
###### Append
`s.push_str("bar");` or `s.push('c');` (`push` can only take one char).
###### Concatenate
```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = s1 + "-" + &s2 + "-" + &s3;
```
`+` operator calls `fn add(self, s: &str) -> String {`.
It works even if `&s2` type is `&String` bc compile can *coerce* `&String` into a `&str` param.
Second argument is a `ref` so s2 will still be valid. But first is not, `s1` will not.

Concatenation can be done without taking any ownership, using a macro: `let s = format!("{}-{}-{}", s1, s2, s3);`.
##### Indexing into Strings
In Rust a letter can be seen either as:
- bytes: [224, 164, 164, 224, 165, 135]
- scalar values: ['त', 'े'] (second value is a diacritic not a char)
- grapheme clusters (the closest thing to letters): [ "ते" ]

`let c = &"hello"[0];` is invalid as it would be a source of many bugs and a performance issue: bc in memory `Strings` are a wrapper over a `Vec<u8>`, `s1[0]` would return the first byte `104` (`'h'` in utf8) and not the first char.
len of `let len = String::from("Здравствуйте").len();` is not 12 (unicode scalar value), it is 24 (bytes needed to encode in UTF-8). 

##### Slice
`let s = &"Здравствуйте"[0..4];` `s -> "Зд"` works but it is not safe: `let s = &"Здравствуйте"[0..1];` Rust would panic at runtime bc the index is invalid.
Solutions:
- iterate over scalar unicode values with `chars`: `for c in "नमस्ते".chars() {`
- iterate over bytes values with `bytes`: `for b in "नमस्ते".bytes() {`
- iterate over grapheme clusters is complex and is provided by crates, not by the standard library.

## Hash Maps (8.3)
Associate a value with a key via an implementation of maps.
