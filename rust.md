# RUST

# Side Notes
- Statements vs expressions: statements perform an action but do not return a value. Expr evaluate to a resulting value. But expressions can be part of statements (ex: `6` expr from `let = 6;` statement).
- Generic type works best for homogeneous collections whether trait bounds works best with multiple possible types.

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
The type `HashMap<K, V>` associates a value with a key via an implementation of maps. This type is not included in the prelude.
`HashMap`s are hashed and therefore DoS resistant (speed / safe balanced algo). But another *hasher* can be specified (`BuildHasher` trait) that can be implemented by hand or called from a public library.

##### Creation
`let mut scores = std::collections::HashMap::new();` creates an empty has map.
`scores.insert(String::from("Blue"), 10);` all the keys / values (distinctly) must have the same type.
```
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```
type of score can not be infered bc `collect()` can return different types.
`teams.iter().zip(initial_scores.iter())` creates a vector of tuples.

##### Ownership
For `Copy` trait types (i32...) values are copied. For owned values (`Strings`...) the `HashMap` values are moved and it becomes the owner unless a ref is used but the variable must live until the `HashMap` dies.

##### Access
`let score = scores.get(&String::from("Blue"));` As `get` returns an option the result will be `Some(&10)`.
`for (key, value) in &scores {` works.
! order is arbitrary.

##### Update
###### By overwriting a value
Simply redo `scores.insert(String::from("Blue"))` will overwrite the old value.
###### By inserting a value only if key does not exist
`scores.entry(String::from("Blue")).or_insert(50);` `entry()` returns an Enum called `Entry`.
`or_insert` keeps the old value if it exists, if not: inserts the param as the new value. And it returns a mutable ref to the new *(or not)* value.
###### By updating the new value working on the old one
```
let score = map.entry(String::from("Blue")).or_insert(0);
*score += 10;
```
blue team, if exists, sees its score updated. `score` has type `&mut V` so it needs to be refererenced to be updated. 

##### Print
`println!("{:?}", scores);` will output `{"Yellow": 50, "Blue": 10}`.

# Error Handling (9)
Rust does not have exceptions and instead distinguishes 2 types of errors:
- *unrecoverable* errors
    Set by the dev
    Stop the execution of the program
    Covered by the usage of the `panic!` macro
- *recoverable* errors
    ex: file not found.
    Problem is reported to the user but the program keeps running
    Covered by the usage of the `Result<T, E>` type

## Unrecoverable Errors with panic! (9.1)
When the `panic!` macro executes program prints a failure message, clean the program (*unwinds*) then quit it.
! *abort* which quits without unwinding and lets the operating system cleaning the memory.
`panic!` result can be set to *abort* in the *Cargo.toml* file.
```
[profile.release]
panic = 'abort'
```
A call to the `panic!` macro causes this message: `thread 'main' panicked at 'crash message set', src/main.rs:2:4`.
When the `panic!` calls comes from an external library (!= our source code) we will need a backtracker to find the origin bc the error message points to the lib.
Example: `let v = vec![1, 2, 3];` if we try to access a non-existant value (`v[99]`) in C we would hit a memory that is not ours (*buffer overread*). In Rust will panic: `thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /checkout/src/liballoc/vec.rs:1555:10`
Running `RUST_BACKTRACE=1 cargo run` with debug symbols enabled (meaning `--release` flag is not set) allows to catch the error from our own code: `11: panic::main at src/main.rs:4`.

## Recoverable Errors with Result (9.2)
`Result` definition (`T` and `E` represent the value returned depending on the case):
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
Example: `open()` returns a `Result` value. If it succeeds f will contain an instance of `Ok` that contains the `std::fs::File` (file handler). If it fails f will contain an instance of `Err` that contains a `std::io::Error` type.
```
fn main() {
    let f = std::fs::File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}
```
Tip: giving to the compiler a wrong return type (`let f : u32 = std::fs::File::open("hello.txt");`) will led to the compiler giving out what was excepted.
`Result::` specifier is not needed bc it is part of the prelude.

`match` alternatives:
- `let f = File::open("hello.txt").unwrap();`. `unwrap` will either return directly the result of `Ok` or automatically call the `panic!` macro.
- `let f = File::open("hello.txt").expect("Failed to open hello.txt");`. `expect` is similar to `unwrap` with a provided `panic!` message (easier to know where the error comes from).
##### propagating errors
IE return the error to the calling code instead of handling it within the function.
###### The `?` operator
```
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) // called if everything works fine
}
```
This code can be shortened as:
```
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
```
actually all this could be shortened to returning `fs::read_to_string("hello.txt")` which opens the file by itslef.
`?` is a nice shortcut. (It calls the `from` trait which converts error types, *so what ?*).
Both `open()` and `read_to_string()` output a `Result`. If `Err`: function returns the error, if `Ok` it keeps going through the code.
! `?` can only be used inside a function that returns a `Result` (`fn -> Result<String, io::Error> `) or it will not compile. Note that even the main can return a `Result`: `fn main() -> Result<(), Box<dyn Error>> {`.

## To panic! or not to panic! (9.3)
Compiler does not understand logic. So when a code has 0% chances to have an `Err`, `unwrap()` (returns the value from `Ok` or panics if `Err`) with no check is a good practice.
`unwrap` (or `expect`) is a good practice when you have not decided yet how to deal with the error.
##### Guideline
`panic!` when code is in a *bad state*, ie:
- When a value is invalid AND either: not expected to happen or code relies on being stable at this point or there is no good way to encode the information or it exposes to vulnerabilities
- When someone uses your code and passes a values that does not make sense
- When external code fails and there is no way to fix it ... unless a failure is the expected possibility, then `Result`.

Pre-checks:
- No need to check whether a value has been passed to a function if the param as a type and not an `Option` bc no value = will not compile
- Use unsigned type when possible to ensure only positive values
- In a module:
    - implement a `new` method that does all checks and returns the good type(s). If not, `panic!` bc the contract is broken. Regrouped and secured checks (at creation).
    - implement public setters/getters and keep variables private, so that only the setter does a check when editing a value (getter example: `pub fn value(&self) -> i32 { self.value }`)


# Generic types, traits, and lifetimes (10)

## Generic data types (10.1)
A given generic type name can represent any given type but once it has been given, it keeps this same type.
##### In functions
`fn largest<T>(list: &[T]) -> T {`: `<T>` before params in the signature is mandatory so the compiler know what `T` means.
Note: `T` (short for *type*) is the common identifier name. The input and output must have the same type but it can be any type.
##### In structs
`struct Point<T> { x: T, y: T, }`. Note that code will not compile if all type `T` fields are not of the same type.
But there can be different *generics*: `struct Point<T, U> { x: T, y: U, }`.
Too many different *generics* might be a sign of wrong design.
##### In enums
`enum Option<K, E> { Ok(K), Err(E), };`. Same thing.
##### In method definitions
In the `Point` struct, implement a `x` method: `impl<T> Point<T> {`. We need to declare `<T>` after `impl` too so the compiler knows that the `T` type from `Point` is a *generic*.
Generic types declared in the implementation methods can differ from the original one, Ex:
```
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```
`impl` generic type declarations can differ from method type parameters but must be the same as struct.
Note that `<T, U>` are declared paired with Struct definition, `<V, W>` are declared after that, in their own relative scope 

##### Performance of code using generics
NO performance difference at runtime between generic and its concrete equivalent type. But takes longer to compile (*Monomorphization* = recreate code with concrete type from the generic form).
```
//generic
enum Option<T> { Some(T), None, }
let integer = Some(5);
let float = Some(5.0);

// monomorphized
enum Option_i32 { Some(i32), None, }
enum Option_f64 { Some(f64), None, }
let integer = Option_i32::Some(5);
let float = Option_f64::Some(5.0);
```

## Traits: defining shared behavior (10.2)
A *trait* tells the compiler about a type's fonctionality. Resembles to *interfaces*
##### Trait definition
```
pub trait Summary {
    fn summarize(&self) -> String; // note the semicolon != brackets
}
```
Any type that has the `Summary` trait must have the `summarize` method with the same signature. Needs to be `pub` so that another crate can implement it
It is implemented as:
```
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```
implementing a trait is similar to implementing methods with `for` added.
It is possible to make a trait local to a scope.
! *coherence* restriction: a trait can be implemented on a type only if either the trait or the type is local to our crate.
If the trait is not local, it needs to be brought into scope as: `use externalcrate::Summary;`

##### Default implementations
It is overwritten by specific methods for a given trait but is still useful. Defined directly in the trait definition (vs semicolon)
```
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```
Here if not overwritten, a default string will be create.
To use the default implementation in an instance, call it empty: `impl Summary for NewsArticle {}`.

##### Traits as arguments
`pub fn notify(item: impl Summary) {` gives access to `item.summarize()` in the fun body.
This form is a sugar for `pub fn notify<T: Summary>(item: T) {` which can be useful to force to have the same types when having 2+ params: `pub fn notify<T: Summary>(item1: T, item2: T) {`.
Several traits can be `impl`: `pub fn notify(item: impl Summary + Display) {` or `pub fn notify<T: Summary + Display>(item: T) {`.
`where` clauses for clearer code: previous function could be written as:
```
pub fn notify<T>(item: T) -> i32
    where T: : Summary + Display
{
```
##### Returning traits
```
fn returns_summary() -> impl Summary {
    Tweet {
        username: String::from("Horse_ebooks"),
        reply: false,
    }
}
```
Says something that implements the `Summary` trait is returned but the exact type is unknown (we are returning an `iterator`).
! If the return type is unsure depending on how the function goes this cannot work.

##### Compare `largest` function with trait bound
To allow comparison and slices largest needs: 
```
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // Copy trait needed to copy a <T>
    for &item in list.iter() {
        if item > largest { // PartialOrd trait needed to compare a <T>
            largest = item;
        }
    }
    largest
}
```
##### Using trait bounds to conditionally implement methods
```
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
Here `cmp_display` is implemented only if `Display + PartialOrd` are implemented.
*Blanket implementations* are extensively used: `impl<T: Display> ToString for T { }`. Here the `to_string()` method can be called on any type that implements the Display trait. That allows to turn int to string because integers implement display (`let s = 3.to_string();`)

## Validating references with lifetimes (10.3)
    Main purpose: prevent dandling references. Rust uses a *Borrow Checker* that verifies the subject of a reference doesn’t live as long as the reference.
Every reference has a *lifetime* (the scope for which it is valid). Often inferred but it sometimes has to be explicit
`&'a mut i32` `'a` is a lifetime parameter. 2+ required to be meaningful, the point is to connect the lifetimes of parameters and return values:
```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
will compile because lifetimes are explicit (and therefore Rust compiler knows they are 'synced'). It would not otherwise. Doing so we are not modifying their lifetime, we are just precising these to the compiler by saying "these three variables will and must at least exist in this scope".
As a result, the compiler thinks the lifetime is equal to the smaller of the lifetimes. So this code will not compile:
```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```
So when to specify lifetime params ? When a reference returned does not refer to a given param (above example: several params at the same time). Ofc if the returned reference refers to a variable created in the function it will not compile anyway (here should be returned the owned data, not a ref).

##### Lifetime annotations in struct definitions
Struct can hold references, but only if they have a lifetime annotation.
`struct ImportantExcerpt<'a> { part: &'a str, }` it means an instance of this struct cannot outline its shortest ref lifetime (`'a`)
##### Lifetime elision rules
There are *input / output lifetimes* (params / return)
Lifetime annotation can be inferred only in `fn` and `impl` cases. If the 3 following rules are applied and a reference does not have a lifetime, the annotation will not be inferred:
- each ref param gets its own lifetime
- if there is exactly 1 ref param its lifetime is assigned to all output lifetime parameters
- if one of several params is `&self` or `&mut self` its lifetime is assigned to all output lifetime parameters
#####  Static lifetime
`'static` is a special lifetime which denotes the entire duration of the program: like all string literals. Should be use only when we want a variable to last the entire program


# Testing (11)
Test body:
- Set up any needed data or state
- Run the code you want to test
- Assert the results are what you expect

## Writing tests (11.1)
##### Test attribute
When `cargo test` is run, Rust runs every `#[test]` function and report whether each test function passes or fails
We can add as many `mod` and `fn` tests as we want
Each test is run in a new thread. If one fails it means the test function panicked.

```
#[cfg(test)]
mod tests {
    #[test] // there can be non-test functions in a test module
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```
`cargo test` outputs:
```
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:
---- tests::another stdout ----
    thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:10:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

    Doc-tests adder // only used for documentation tests
...
```
###### `assert!` macro
Takes a boolean. If it returns `true` test passes but if it returns false `panic!` is called and test fails (remember to negate a function if `false` means no issue)
###### Test equality with the `assert_eq!` & `assert_ne!` macros
`assert_eq!` checks for equality of 2 expr (left|excepted and right|actual), `assert_eq!` checks for inequality (useful when unsure what a value will be but sure what a value should not be)
If one fails the values passed are printed (easier to see why the test failed)
In some cases (structs and enums the programer define) will have to be added `#[derive(PartialEq, Debug)]` to check equality & print the values.
##### Custom failure messages
An optional argument can be added to `assert...!` macros that acts as the `format!` macro (variadic params) as: 
`assert!(result.contains("Carol"), "Greeting did not contain name, value was {}", result);`

##### check for panics
the attribute `#[should_panic]` set a test as failed if `panic!` has not been called
Must be called the line after the `#[test]` attribute.
`expected` allows to check a `panic!` message includes the expected string literal: `#[should_panic(expected = "Guess value must be less than 100")]`. Tester will tell if program panicked & the right substring was printed, or will set the test as failed.

##### Using Result<T, E> in tests
Tester can work with Ok() (passed) / Err() (failed) instead of panics.
`#[should_panic]` does not work with it but it's neat anyway:
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Running tests (11.2)
Note: ` -- ` is a separator. Pre-params apply to `cargo test` & post-params apply to the binary
###### Running tests in parallel or consecutively
`cargo test -- --test-threads=1` means NO parallelism
Takes longer but tests will not interfere with each other if they share a state
Example of several tests writing to a same file at the same times might result in a failed test even if the code is ok
###### Showing function output
By default in the test environment standard ouput (like `println!`) is not printed on passed test but it is for failed test
`cargo test -- --nocapture` allows to print ouput on passed tests too.
###### Running a subset of tests by name
A filtered test can be run by calling any function that has a `#[test] attribute`: `cargo test test_fn_name`
! Tests will run on every test functions that have the param as a substring
! Works with test module names too bc the checked string is `module::fn`
It is possible to set the `#[ignore]` attribute (after `#[test]` line): these will not be tested unless `cargo test -- --ignored` is run, which will ONLY test `ignored` functions

## Test organization (11.3)
- *Unit tests* check every module, even private interfaces
- *Integration tests* check code as an external user would
Both are important to be sure pieces of a program work separately and together

##### Unit tests
Located in the *src* directory in each file, alongside the code they are testing
Convention is to create a `tests` module in each file, annotated with `#[cfg(test)]` which tells Rust to compile it only when `cargo test` is run
In Rust private functions can be tested, they are accessible by default by the testing environment

##### Integration tests
Entirely external to the library. Therefore no need for `#[cfg(test)]` but they can only call public API functions
Create a *test* directory next to *src*, with a (for example) *tests/integration_test.rs* file that contains:
```
use crate_name;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```
When running `cargo test` 3 sections will now appear: unit / integration / doc tests but only integration test can be run: `cargo test --test integration_test` (as the filename). Multiple files can be added (ex one for each functionality tested). Then will appear a new section for each file.
This can be avoid by creating *tests/common/mod.rs*: naming convention makes so these files will not be treated as separate crates, *common* will be treated as a module: adding only `pub fn setup() { }` to `mod.rs` allows to call it from *integration_test* as a module: 
```
use crate_name;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

###### Integration tests for binary crates
Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file.
Doing so allows to have *integration test*. It is not possible to `use` elements of a *src/main.rs* file: binary crates are meant to run on their own

# Functional language features: Iterators and Closures (13)

## Closures: anonymous functions (13.1)
*Closures* are particular anonymous functions that are storable in a variable or passable as function arguments
Unlike functions they capture their environment (can access parents scope variables)
light syntax for inline closure: `let add_one = |x| x + 1 ;` ; full syntax:
```
let expensive_closure = |param1, param2| { // |param1: u32, param2: u32| -> u32 will compile too
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    param1
}; // semicolon needed to end the statement
```
`expensive_closure` contains the definition of the function not its result. It is called the same way a function is
Annotate types is not needed (but possible), types are infered bc unlike functions closure are not interfaces exposed to users
! The compiler infers types & traits at first call so recalling a closure with different param types will not compile

###### Closures capture their environment
Closure have, like functions, at least on of these traits that are inferred depending on what a closure does of its environment variables:
- `FnOnce` takes ownership and consumes the environment variables, meaning this closure can only be called once
    All closures implement it
    To take ownership, use the `move` keyword before the parameter list (`let equal_to_x = move |z| z == x;`)
- `FnMut` same but mutably: can change the environment
    Implemented if the closure do not move the captured variables
- `Fn` borrows values from the environment immutably
    Implemented for closures that do not need mutable access to the captured variables

###### Storing closures and their result
This is known as *cache, memoization, lazy evaluation*. The solution is to store both the closure and its result in a struct. Whatever their signature is closures will have different types so solution is to consider them generics. 
Closures are functions-like: one+ of `FnOnce` `FnMut` or `Fn` traits must be implemented too.
```
struct Cacher<T>
    where T: Fn(u32) -> u32 // every closure will have to take a u32 and return a u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                if v == arg {
                    v
                } else {
                    self.value = (self.calculation)(arg);
                }
            },
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```
and called as:
```
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        println!("Today, run for {} minutes!", expensive_result.value(intensity));
    }
}
```
Issue: value will be updated depending on the arg, could use a hashMap to store several results

## Processing with iterators (13.2)
*Iterators* are a way of processing a series of elements. THey implement an `Iterator` trait and are defined as:
```
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; // unique mandatory function to define
    // methods with default implementations elided
}
```
`iter()` takes immutable references to the values, but `into_iter()` takes ownership and returns the owned values. If references are mutable, `iter_mut()` can be used

##### Methods
- Methods that consume the iterator = all methods that call `next()` like `sum()`: `let total: i32 = vec![1, 2, 3].iter().sum();`
- Methods that produce other iterators = *iterator adaptors*: methods that changes iterators like `map()`: 
```
let v1: Vec<i32> = vec![1, 2, 3];
let v2 = v1.iter().map(|x| x + 1).collect();
```
`collect()` has to be called since `map()` does not consume the iterator (just updates it). `collect()` consumes the iterator and returns a collection of the new data

###### Iterators + closures: common use case
```
#[derive(PartialEq, Debug)]
struct Shoe { size: u32, style: String, }

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

let in_my_size = shoes_in_my_size(shoes, 10); // filters out size: 13 shoe from the vecArray
```

##### Creating iterators
```
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```
This offers access to `.map()`, `.filter()`, `.sum()`, `.zip()`, `.skip()` etc, methods of `Iterator`

## Performances: loops vs iterators (13.4)
Iterator are a *zero-cost abstrations* feature bc it is compiled down to low-level code. Rust *unrolls* loops.
Iterators are slightly faster than loops


# More about Cargo and Crates.io

## Customize builds with release (14.1)
Cargo has 2 main profiles: `dev` (used with `cargo build`) and `release` (`cargo build --release`)
In *Cargo.toml* can be added [profile.*] sections as for example:
```
[profile.dev]
opt-level = 1 // [0..3]
```

## Publish libraries to crates.io (14.2)
Make useful documentation comments: `///` which support Markdown format
Running `cargo doc --open` will generate a documentation from project code comments.
Ex: 
```
/// Adds one to the number given.
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
will create a doc with a side panel for the function name with information, signature, examples, etc. Common sections: Panics, Examples, Errors, Safety
Running cargo test will run the code examples in your documentation as tests
`//!` To add top-level (crate) doc comments. If put after a `///` it will add into the top-level doc section relative to a function

##### Exporting as a Public API with `pub use`
Internal structure can differ from public API
`pub use` allows to avoid `use my_crate::some_module::another_module::UsefulType;` and rather do: `use my_crate::UsefulType;`
A crate can have a nice hierarchy that is not convenient for public users: it is not requiered to (re)design it: `pub use` removes the internal organization from the public API and re-exports an item at the top-level. 
Better to do this: `pub use kinds::PrimaryColor;` which re-exports and is accessible doing `use art::PrimaryColor;` ; than this:
```
pub mod utils {
    use kinds::*;
```
which is obscure for a public user (and hidden from the doc).

##### Set up crates.io account
Create account on site, get an API token then do `cargo login API_KEY_VAL` (stored in *~/.cargo/credentials*) to be allowed to publish crates

##### Crate metadatas
*Cargo.toml*
```
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"
description = "A fun game where you guess what number the computer has chosen." // display in search results
license = "MIT OR Apache-2.0" // multiple licences is ok
```
If it is not a SPDX licence, create a *licence-file* and specify its name in the `licence=` field

##### Publish, update, remove
`cargo publish`
To update a package: update the `version=` field from the *Cargo.toml* file (follow this convention *https://semver.org/*) then re-run `cargo publish`
`cargo yank --vers 1.0.1` which can be undone: `cargo yank --vers 1.0.1 --undo` prevents every new project to starting to depend to that version. But projects that already used that version will continue to depend on it with no issue

## Organize large projects with worspaces (14.3)
It is the possibility to split up a package into multiple library crates that share the same *Cargo.lock* and output directory
Several wyas to structure it, idiomatic one:
library crate 1: `add_one` function
library crate 2: `add_two` function
binary crate depends on crate 1 & 2 and provides main functionality
! There will only be a single, shared, *target* directory so not every crate has to be rebuilt every time

##### Creation
Create a folder for each *workspace* with a *Cargo.toml* file as:
```
add
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```
*adder/Cargo.toml* file:
```
[workspace]

members = [
    "adder",
]
```
then run `cargo new adder` from the *adder* dir.
Update top-level *Cargo.toml* as:
```
[workspace]
members = [
    "adder",
    "add-one",
]
```
run `cargo new add-one --lib` from the *add-one* lib dir
Add a `pub fn add_one()` in the *add-one/src/lib.rs*
Add to *adder/Cargo.toml* the following:
```
[dependencies]
add-one = { path = "../add-one" }
```
In *adder/src/main.rs* add-one can now be called as:
```
use add_one;

fn main() {
    println!("{}, add_one::add_one(10));
}
```
Now it is possible to ruin `cargo build` in the top-level *add* directory... and run it with `cargo run -p adder`

###### Add external crate to a workspace
To make an external crate available to a workspace with `use cratename;`, add this to *workspace/Cargo.toml* file:
```
[dependencies]
rand = "0.3.14"
```

###### Add testing to a workspace
In *add-one/src/lib.rs* add:
```
pub fn add_one(x: i32) -> i32 { /// already added
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```
Run `cargo test` in top-level *add* dir will test sub-workspaces too or `cargo test -p add-one` to test a specific crate

###### Publish to crates.io
run `cargo publish` on every crate dir, not just on the top-level

## Install and run binaries from crates.io (14.4)
`cargo install` allows to install and use binary crates locally. They are located 
Crate can either be a library (not runnable but suited to fit in other programs), has a binary target (runnable program if there is a *src/main.rs*) or both
If a binary is in the `$PATH`, it can be run with `cargo somebinary`


# SMART POINTERS (15)
Data structures that have useful methods, have their core data on the heap and can share ownership.

## Box to point to data on the heap (15.1)
Boxes do not have capabilities, simple and light (no overhead) smart pointer. They implement the `Deref` and `Drop` traits.
###### Allocation
`let b = Box::new(5);` b contains a pointer to the value `5(i32)` allocated on the heap. Like stack data both the data and the pointer will be deallocated once out of scope
###### The `Cons` list
Receives a pair of arguments, the value of the current item and the next item. The last item has a `Nil` value without a next item
Not common in Rust to treat list of items (`Vec<T>` is better most of the times)
```
enum List {
    Cons(i32, List), //  will not compile: Cons(i32, Box<List>), would work
    Nil,
}
```
This list is recursive so rust cannot guess its type and the compiler will not compile. Using a `Box<List>` would work because only a pointer will be stored on the stack, the unknown size (list) will be on the heap

## `Deref` trait to treat smart pointers like references
Implementing the `Deref` trait allows to customize the dereference operator (`*`), therefore allows for smart pointers to work like a reference. It allows to deference something that is not a ref. Derefencing (`*`) means pointing to the value of a reference (like in C).
```
let x = 5;
let s = &x;
let h = Box::new(x);

assert_eq!(x, *s); // are equal
assert_eq!(*s, *h); // are equal, would not be possible to use `*` without the Deref trait
```

###### Implement the `Deref` trait on a struct
```
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
```
So doing `*h` == `*(h.deref())`
Note that `deref()` has to return a ref and not a value so `self` keeps the ownership

###### *deref coercion*
Happens automatically when a ref to a type's value is passed to a function that was waiting for another type of parameter. It converts into the right type using `deref()`. So code can work bot with references and smart pointers. Like `&MyBox<String>` -> `&String` -> `&str`. With/out *deref coercion*:
```
let m = MyBox::new(String::from("Rust"));
hello(&(*m)[..]); // without
hello(&m); // with
```
`*m` dereferences `MyBox<String>` into `String` and `[..]` turns the `String` into a string slice.

###### mutable *deref coercion*: `DerefMut`
When `T: DerefMut<Target=U>` is implemented, `&mut T` will be converted to `&mut U`. But can too convert `&mut T` to `&U`: Rust can coerce a mutable into an immutable ref (not the opposite as it would break the borrowing rules)

## Running code on cleanup with the `Drop` Trait
= customized code ran when a value gets out of scope. Ex: `Box<T>` customizes `Drop` to deallocate the space on the heap

###### Implementation
```
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Instance drops!");
    }
}
```
`println` will be called when an instance is out of scope. No need to bring `Drop` into scope as it is in the prelude
Variables are dropped in the reverse orer of their creation

##### Drop a value early
Can be useful when using locks for example. Calling `Drop::drop()` would not compile, `std::mem::drop()` needs to be called instead. It takes the variable to drop as a parameter and call the `Drop::drop()` function. Do not be scared to drop too early, Rust checks valid references so the compiler would yell.

## `Rc<T>`: reference counted smart pointer
Design to work with multiple ownership pointing to a heap data and it cannot be determine a t compile time which part will finish using the data last. Its purpose is to keep count of every active reference to determine whether or not a value is still in use.
Allows to have multiple immutable references (that point to mutable data on the heap ?)
Note: `Rc` works only on single-threaded scenarios.
```
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // unidiomatic: a.clone()
    let c = Cons(4, Rc::clone(&a)); // unidiomatic: a.clone()
}
```
Each call to `Rc::clone()` will increase the count, and the data will only cleaned up once count = 0. Count is automatically decreased
`Rc::strong_count(&a)` returns the active count

## `RefCell<T>` and the interior mutability pattern
*Interior mutability* is a design pattern that allow mutability of data attached to immutable references. Uses `unsafe` code to workaround Rust's rules. Single ownership to the data. Useful when you can guarantee your program will work but the compiler cannot ensure it.
Borrowing rules: a program can have either (but not both of) one mutable reference or any number of immutable references. References must always be valid. But unlike `Box<T>`, for `RefCell<T>` borrowing rules are applied at *runtime* rather than *compile time* and will cause to panic and exit. Consequence: slower and not errors/memory-safe.
Rules reminder: `RefCell<T>` allows to have many immutable borrows OR one mutable borrow at any point in time.
If two mut are borrowed panic message will be: `already borrowed: BorrowMutError`

##### Interior Mutability: A mutable borrow to an immutable value
If a function uses a trait that does not allow mutability but mutability is required. `RefCell` will wrap the variable and allow for mutability by calling `borrow_mut()` (returns a `RefMut<T>`) and `borrow()` (increases its count and returns a `Ref<T>`) methods
Works well for *mock objects*

##### Having multiple owners of mutable data by combining `Rc<T>` and `RefCell<T>`
`Rc<T>` allows to have multiple owners who are limited to an immutable access... Unless `Rc<T>` holds a `RefCell<T>`. Would look like:
init a list of: `Rc<RefCell<i32>>;`, `let value = Rc::new(RefCell::new(5));` and create clones of `let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));` then `*value.borrow_mut() += 10;`: every clone has the same `value` value as `a`
Interior mutability can also be provided by `Cell<T>` (which copies the value instead nof giving references to the inner value) or `Mutex<T>` (which is safe across threads)

## Reference cycles can leak memory
Ex: using `Rc<T>` and `RefCell<T>` where items refer to each other in a cycle = memory leaks, ref count will never reach 0 -> values will never be dropped:
```
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
}
```
Solution: pay attention to this as there is no Rust safety or have cycles made up of some ownership and of some non-ownership relationships, and only the ownership relationships affect whether or not a value can be dropped

##### Prevent reference cycles by turning an `Rc<T>` into a `Weak<T>`
It is also possible to create a *weak reference*(`Rc::downgrade` creates a `Weak<T>` that increases the `weak_count`: unlike `strong_count` the `Rc` instance can be cleaned up even if `weak_count` is not 0). So creating weak ref cycles is safer as it will be broken once strong reference comes down to 0. But since `Weak<T>` value pointer might have been dropped it has to be checked by calling

###### Create a tree data structure: a `Node` with child nodes
```
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

```
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```
Doing this allows parent to own a mutable reference to the child Node and a `Weak<T>` ref (does not own) from the child to the parent so that parent can drop child (not the other way) and there is no refence cycle

# FEARLESS CONCURRENCY
Rust handles threads as a 1:1 relation with operating system threads but M:N relation has been made possible through crates.
Issues Rust ownership / concurrency system prevents from happening:
- *Deadlocks* = both threads are waiting for each other preventing both threads from continuying
- *Race conditions* = threads are accessing data in an inconsistent order
- Bugs hard to reproduce

## Threads
`thread::spawn()` takes a closure and executes code in a new thread
```
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
will print every `handle` then main thread.
If `handle.join()`had been called after the main for there zould have been 2 `println!` running concurrently.
Without `handle.join()` the program would have ended with the main thread, without waiting for the `handle` thread to end.

##### Ownership system of threads
`move` closure trait allows to borrow or take ownership (?) of variables called within the closure (`let handle = thread::spawn(move || {`)
Be careful, a variable passed to a `move` thread cannot be accessed in its former context

## Using message passing to transfer data between threads
Channels allow to send messages from several producers to an end
```
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
Note that receiving can be done with `recv()` (will block the main thread's execution and wait for a value) and `try_recv()` (non-bloquant version of `recv()` that returns a `Result<T>
```
let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx); // tx1 becomes another producer of messages alongside tx
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx { // acts as blocking recv() calls
    println!("Got: {}", received);
}
```
The `for` from the main will wait for messages to come and will print both thread messages concurrently
! Once a variabe has been sent its ownership is transmitted, therefore it is no longer accessible in the current scope / thread

## Shared-state concurrency
##### Mutexes: allow access to dat from one thread at a time
Mutex is a shortcut for mutual exclusion: it only allows access (asking for the lock) to some data one thread at a time
Usage rules:
- Attempt to access the lock before using the data
- Unlock the data after using it so other threads can acquire the lock
Rust advantage: no risk to get un/locking wrong
Mutexes provide interior mutability like `RefCell<T>` so mutex is immutable but we can get a mutable ref of the data it contains
! Mutexes can lead to deadlocks (two threads waiting for each other forever bc 2 locks are needed for them)
##### `Mutex<T>` (smart pointer) API
```
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```
`.unwrap()` is important to make the thread panic if a lock fails
`.lock()` returns a smart pointer called *MutexGuard* with the `Deref` trait to point to inner data + `Drop` trait to release the lock automatically when a *MutexGuard* goes out of scope
##### Share `Mutex<T>` between multiple threads
```
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
Will not compile bc Rust prevents from moving *counter* (the mutex) ownership into multiple threads
Could solution be: `Rc<T>` to have multiple owners, remplacing `let counter = Mutex::new(0);` by `let counter = Rc::new(Mutex::new(0));` and calling `let counter = Rc::clone(&counter);` on each iteration ?
No, it would not work bc `Rc<T>` is not safe to share across threads (reference count calls are done non-concurrently so they could be interrupted by another thread so Rust prevents it)
Real solution: `Arc<T>`
##### Atomic reference counting with `Arc<T>`
Atomic work just like primitives (here `Rc<T>`) but are cross-thread safe and come with a performance cost
Final result: 
```
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

## Extensible concurrency: `Sync` and `Send` traits
Rust has very few built-in concurrency features so look at crates / standard library / yourself
Built-in are `std::marker` traits `Sync` and `Send`

##### Allowing transference of ownership between threads with `Send`
Indicates that the ownership of the type implementing `Send` can be transferred across threads.
Almost every type *is* `Send` (ie implements `Send`). Exceptions: raw pointers; `Rc<T>` to avoid 2 threads updating the reference count at the same time. So Rust prevents using `Rc<T>` across threads, erroring: `the trait Send is not implemented for Rc<Mutex<i32>>`
##### Allowing access from multiple threads with `Sync`
Indicates that it is safe for a type that is `Sync` to be referenced from multiple threads
If a `&T` is `Send` then it is `Sync` too
Almost every type is `Sync` too (every primitives). Exceptions: `Cell<T>` related types (bc of runtime borrowing impl is not thread-safe), `Rc<T>` (bc of the same reason `Send` is not)
##### Implementing `Send` and `Sync` manually is unsafe
Bc if a type does not come with it means something, these types are not made up for this

# Object oriented programming features of Rust (17)
It is a design pattern in which objects pass messages to each other
Rust is influenced by many programming paradigms including OOP and functional

## Characteristics of object-oriented languages (17.1)
Objects, encapsulation, inheritance are features OOP / Rust share
Depending on the definition Rust is/not an OOP language or not

##### Objects contain data and behavior
OOP definition (one of many) that tends to define Rust as OOP (objects = `struct` & `enum`, since `impl` block provides methods):
> "Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations."

##### Encapsulation that hides implementation details
Polymorphism != Inheritance: polymorphism = general concept: code that code work with data of multiple types
Encapsulation means only the public API of an object can be reached, its internals are not and can be changed transparently.
Rust feature allowing encapsulation is the `pub` keyword, by default modules types functions and methods are private

##### Inheritance as a type system and as code sharing
Inheritance meaning can be either: reuse of code or polymorphism (code that can work with data of multiple types)
There is no inheritance feature in Rust but traits allow to share methods implementation and generics abstracts over possible types (*bounded parametric polymorphism*)

## Using trait objects that allow for values of different types (17.2)
Example: a `gui` crate that allows to draw components such as `Button`, `Image`, `SelectBox` or whatever the user would want to integrate
An OOP implementation would be to define a parent class named `component` that implements a `draw` method. Its children would be `Image` etc and inherit, overwrite `draw`. Rust equivalent:

##### Define a trait for common behavior
Parenthesis: `enum` and `struct` have methods and data separated but traits are more like objects as they combine data and behavior. But they have a specific purpose: allow abstraction across common behavior

Define a `Draw` trait that will have a `draw` method: 
```
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```
`Vec<Box<dyn Draw>>>` stands-in for any type inside a Box that has to implement the `Draw` trait. Generic types would not work here bc only 1 type could be called (Buttons for example) whereas at runtime traits allow for multiple concrete types.

To add something to draw, simply declare a struct:
```
use gui::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```
If a `Box::new()` call does not implement the `Draw` trait it will not compile

## Implemting an object-oriented design pattern: the *state pattern*
*state object* means a value has both an internal state and a private value that can only be updated through its public methods. Each method updates the state value which is `Some(<Box>)`
Example: a `Post` crate that allows to create a Post draft, wait for its review and publish it once it has been reviewed. The post cannot be published before, and its content will be empty until publication
```
use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    post.request_review();
    assert_eq!("", post.content()); // post content is still empty
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content()); // post has been published
}
```
Here we can only access the `Post` type, that has an internal value and three mathods that are the only way to update the instance state and content changes
definition of public struct Post:
```
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        ""
    }
    
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { // if state != none: take the value out of the option, put it in `s` and consume the current state ...
            self.state = Some(s.request_review()) // ... update and return a new state
        }
        // cannot do it in a one line operation as we need to get ownership of the `state` value
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // the method is only valid when cvalled on a `Box` holding the type
    // the method takes ownership of the `Box<Self>` (state value) to update the state of the `Post`
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

# PATTERNS AND MATCHING (18)
Patterns are a syntax for matching against values. If the value does not fit the shape of the pattern, the code associated with the pattern will not run
## Use cases (18.1)
##### `match` arms
! must be exhaustive (all possible expressions must be accounted for. Catchall patterns like `_` help)
```
match VALUE { // match x {
    PATTERN => EXPRESSION, // 1 => println!("one"),
    PATTERN => EXPRESSION,
}
```
`_` will match anything but will never bind to a variable
##### Conditional `if let` expressions
Mainly is a shortcut for a `match` of only one case
Compatible with `else`, `else if` or even `else if let`
```
let favorite_color: Option<&str> = None;
if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
}
```
Shows the correct syntax but condition is not met.
Note that `if let` creates a scope, so be careful to shadowing
`if let` downside: exhaustiveness is not checked (ie `else` is not mandatory) which is a source of logical bugs
##### `while let` conditional loops
```
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```
When `pop()` returns `None` the loop stops (pop is a `Vec` method that deletes last elem and returns `Some(new_last_elem)` or None` if empty).
##### `for` loops
```
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() { // enumerate produces a tuple from an iterator
    println!("{} is at index {}", value, index);
}
```
##### `let` statements
It is actually a pattern: `let PATTERN = EXPRESSION;`
Destructuring (here a tuple) is possible: `let (x, y, z) = (1, 2, 3);`. Note that `let (x, y) = (1, 2, 3);` will not compile
##### function parameters
`fn foo(x: i32) {` is a pattern
Destructuring is also possible:
```
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

## Refutability: whether a pattern might fail to match (18.2)
Patterns can either be:
- Irrefutable: ie they match for *any* possible value. `let x = 5;` -> x can hold every value possible. Function params, `let` statements, `for` loops can only accept irrefutable patterns bc otherwise programs could not work safely
- Refutable: ie some values are not handled. Ex: `if let Some(x) = val` if `val` equals `None` pattern will not match. `if let` and `while let` can only accept refutable patterns bc they are made to handle possible failure
Consequence: `let Some(x) = some_option_value;` cannot compile and would error `refutable pattern in local binding: 'None' not covered`. `if let` is nice when  we have a refutable pattern where an irrefutable pattern is needed
The other way other does not make sense and will not compile: `if let x = 5;` will error: `irrefutable if-let pattern`

## All the pattern syntax (18.3)
##### Matching literals
`match x { 1 => println!("one"), }`
##### Matching named variables
It is actually an issue bc match starts a new scope so var names will be shadowed
```
let x = Some(5);
let y = 10;

match x {
    Some(y) => println!("Matched, y = {:?}", y),
    // y is a new variable that can take any value, will match wil x (Some(5)) and will print y = 5
}

println!("at the end: x = {:?}, y = {:?}", x, y); // will output x = Some(5), y = 10
```
Solution: use *match guards* conditional (later section)
##### Multiple patterns and ranges of values
`|` syntax means *or*.
`...` allows to match an inclusive range of values. Only allowed for numeric values and `char` values.
```
match x {
    1 | 2 => println!("one or two"),
    2 ... 5 => println!("2 | 3 | 4 | 5"),
    _ => println!("anything else"),
}
```
##### Destructuring to break apart values
Patterns can be used to destructure structs, enums, tuples and references, even nested
complex example: `let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });`
###### Destructuring structs and enums
```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p; // shorthand for let Point { x: a, y: b } = p; 
}
```
Can be combined with `match`:
```
match p {
    Point { x, y: 0 } => println!("On the x axis"), // matches for all x, when y: 0
    Point { x, y } => println!("Coucou"), // matches everything
}
```
###### Destructuring enums
```
enum Color { // tuple
   Rgb(i32, i32, i32),
   Hsv(i32, i32, i32)
}
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // struct
    ChangeColor(Color), // enum
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => println!("Quit variant: no data to destructure."),
        Message::Move { x, y } => println!("move struct destructured"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("colors enum(tuple) destructured"),
    }
}
```
###### Destructuring references
When the pattern's value contains a reference, specify a `&` in the pattern to get the pointed value rather than the variable that hosts the ref
```
let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
];

let sum_of_squares: i32 = points
    .iter()
    .map(|&Point { x, y }| x * x + y * y)
    .sum();
```
Without the `&` in `|&Point { x, y }|`: compile error, cannot access Point values directly

##### Ignoring values in a pattern
`_` replaces 1 value and `..` replaces every values left (but listed ones)
Can be used in any pattern
###### Ignoring an entire value with `_`
- functions params: `fn foo(_: i32, y: i32) {` can be useful when implementing a trait if a param is not needed (clearer and removes the compiler warning)
- `match`: 
```
match (setting_value, new_setting_value) {
    (Some(_), Some(_), Some(_)) => () // useful to check to values are not `None`
    (first, _, third) => (),
}
```
###### Ignoring an unused variable by starting its name with `_`
Useful when prototyping: `let _x = 5;` prevents from a compiler warning.
Note that `_s` binds, `_` does not. Important:
```
let s = Some(String::from("Hello!"));
if let Some(_s) = s {
    println!("found a string");
}
println!("{:?}", s);
```
here the value has been moved into `_s` so compiler error when trying to print it. Would not have been moved using simply `_`
###### Ignoring remaining parts of a value pair with `..`
```
match numbers {
    (first, .., last) => {
```
However `(.., second, ..)` this pattern would result in an error as it is unclear.
##### Extra conditionals with *match guards*
Additional `if` in a `match` case, which is actually not a pattern
```
match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
```
Note: if `if` condition is not matched code will keep itering cases
Note2: a shadowing workaround as you can use outer variables in match guards
Match guard can be used with `|`:
```
let y = false;
match x {
    4 | 5 | 6 if y => println!("yes"),
```
that acts as `(4 | 5 | 6) if y` which means the `if` always applies to the result of the entire arm pattern
###### `@` bindings
Allows to create a variable that at the same time holds a value and tests it
```
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3...7 } => println!("Found an id in range: {}", id_variable),
    // id value is both saved into id_variable AND checked on the 3..7 range
    Message::Hello { id: 10...12 } => println!("Found an id in another range"), // cannot access id value here
```
##### Legacy patterns: `ref` and `ref mut`
Mostly useless as of today bc Rust has been updated with abstraction but was useful when borrowing a variable, prevented the variable to be moved
`ref` creates a reference, it is the opposite of `&` in patterns: it binds to a `&` so that Rust does not try to moove a variable.
Old version working code:
```
let robot_name = &Some(String::from("Bors"));
match robot_name {
    &Some(ref name) => println!("Found a name: {}", name),
    None => (),
}
```
Today's changes a lot easier, in match first arm: `Some(name)`


# ADVANCED FEATURES (19)
## Unsafe Rust (19.1)
Allow it with `unsafe` before a block
The borrow checker etc still stands but enforces Rust to deal with memory at runtime. It gives 4 superpowers:
- Deref a raw pointer
- Call an unsafe fun / method
- Access / modify a mut static variable
- Implement an unsafe trait
Beware, it leaves the door open to problems due to memory unsafety, such as null pointer dereferencing
Made possible bc Rust prefer to reject valid programs rather than accept invalid ones so it has to allow a workaround ; to allow low-level programming such as directly interacting with the operating system or even writing your own operating system
Wrapping unsafe code in a safe abstraction prevents uses of `unsafe` from yourself or other users

##### Dereferencing a raw pointer
Raw pointers: `*const T` and `*mut T`. The `*` is not the dereference operator, it is part of the type name
In the context of raw pointers, *immutable* means that the pointer can’t be directly assigned to after being dereferenced.
Different from references and smart pointers, raw pointers:
- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location (be careful to data races)
- Are allowed to be null
- Are not guaranteed to point to valid memory
- Do not implement automatic cleaning
Creating a raw pointer (safe code):
```
let mut num = 5;
let address = 0x012345usize;

let r1 = &num as *const i32; // created from a ref so guaranteed to be valid
let r2 = &mut num as *mut i32; // created from a ref so guaranteed to be valid 
let r3 = adress as *const i32; // created from an arbitrary memory location, unsecure

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```
Creating a pointer does no harm, only accessing to its value can. In safe code it is not possible to read the data pointed at by a dereferenced raw pointer
Major raw pointer use case: interfacing with C code or building up safe abstractions that the borrow checker doesn’t understand
##### Calling an unsafe function or method
definition of a fun: `unsafe fn dangerous() { ... }` which must be called wrapped in an unsafe block
To create a safe abstraction over unsafe code just wrap the unsafe part in a fun
##### Using `extern` functions to call external code
extern facilitates the creation and use of a *Foreign Function Interface (FFI)* to interact with other languages
Extern implies the `unsafe` keyword
```
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
The `"C"` calls the language corresponding *application binary interface (ABI)* (assembly)
Work the other way too, the following function is accessible in C code:
```
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```
##### Access or modify a mutable static variable
*global variables* exist in Rust as *static* variables but can cause data races when mutable
```
static HELLO_WORLD: &str = "Hello, world!"; // type is &'static str

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```
Variable's type must be annotated. All static references have the `'static` lifetime so it can be implicit
Static variables have a fixed adress in memory and are mutable (!= constants).
Any code that reads or write from a static variable must be in an `unsafe` block bc it is difficult to ensure there is no data race in a globally accessible variable
Avoid mutable static variables whenever possible
##### Implementing an unsafe trait
A trait is `unsafe` when any of its methods has some invariant Rust compiler cannot verify. 
```
unsafe trait Foo { // Declaration
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

## Advanced Lifetimes (19.2)
## Advanced traits (19.3)
## Advanced types (19.4)
## Advanced functions & closures (19.5)

## Macros (19.6)
Macros refer to a family of features:
- *Declarative* macros with `macro_rules!`
- *Procedural* macros with: custom `#[derive]` macros ; attribute-like macros ; function-like macros
##### The difference between macros and functions
Macros are *metaprogramming* = write code that writes other code
Downside: macro definitions are more complex, less readable and maintainable than fun as we write Rust code that writes code
Macros can take a variable number of params
Fun are called at runtime but macros are expanded before. So for example traits can be implemented in macros, not functions
Macros must be defined/brought into scope before being called. Fun can be defined and called anywhere
##### Declarative macros with `macro_rules!` for general metaprogramming AKA *macros by example*
Most common. Similar to pattern matching: at compilation type, macros compare a value (associated to code) to patterns (structures of that source code) then run the code associated with the pattern
`let v: Vec<u32> = vec![1, 2, 3];` 
Simplified definition of the macro `vec!`:
```
#[macro_export] // allows to bring the macro into scope
macro_rules! vec { // macro_rules! name_of_the_macro
    ($($x:expr), *) => { // pattern
        let myt temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
}
```
Here there is a one-armed pattern (`($($x:expr), *)`). if it matches: associated code is emitted, if not: error




# APPENDIX (21)
## Operators and symbols (21.2)
##### Operators
- `ident!` followed by `()/{}/[]` - macro expansion
- `&expr, &mut expr` - Borrow
- `&type, &mut type, &'a type, &'a mut type` - Borrowed pointer type
- `!expr` - Bitwise or logical complement
- `expr & expr` - Bitwise AND 
- `var &= expr` - Bitwise AND and assignment
- `expr ^ expr` - Bitwise exclusive OR
- `expr | expr` - Bitwise OR
- `expr << expr` - Left-shift
- `expr >> expr` - Right-shift
- `*expr` - Dereference
- `*const type, *mut type` - Raw pointer 
- `.., expr.., ..expr, expr..expr` - Right-exclusive range
- `..=expr, expr..=expr` - Right-inclusive range
- `ident @ pat` - Pattern binding
- `pat | pat` - Pattern alternatives
- `expr?` - Error propagation
##### Symbols
- `...u8, ...i32, ...f64, ...usize, etc.` - Numeric literal of specific type
- `"..."` - String literal
- `r"...", r#"..."#, r##"..."##, etc.` - Raw string literal, escape characters not processed
- `b"..."` - Byte string literal; constructs a [u8] instead of a string
- `br"...", br#"..."#, br##"..."##, etc.` - Raw byte string literal, combination of raw and byte string literal
- `'...'` - Character literal
- `b'...'` - ASCII byte literal
- `|...|expr` - Closure
- `!` - Always empty bottom type for diverging functions 

## Derivable traits (21.3)
the `derive` attrb appliable to a struct / enum definition implements traits. List of `derive` traits available: 
##### `Debug` for programmer output
- What it does: enable to print instances of a type for debug
- Operators / methods made available: `:?` within `{}` operator
- Trait required for: `assert_eq!` macro
##### `PartialEq` and `Eq` for equality comparisons
- What it does: compare instances of a type. `Eq` signals that for every value of a type the value is equal to itself.
- Operators / methods made available: `==` `!=` operators, `eq()` method
- Implementation requirments: `Eq` requires `PartialEq` trait and even then, some types cannot implement `Eq` (floating point types)
- Trait required for: `Eq` required for keys in a `HashMap<K, V>`
- Notes: `PartialEq` implements `Eq` ; `PartialEq` on structs: equal only if all fields are. On Enums: only equal to the compared variant
##### `PartialOr` and `Ord` for ordering comparisons
- What it does: compare instances of a type for sorting purposes. `Ord` allows to know that a valid ordering exists between 2 values
- Operators / methods made available: `>` `<` `<=` `>=` operators, `partial_cmp()` method which returns an `Option<Ordering>` ; `cmp()` method which returns an `Ordering`
- Implementation requirments: `Ord` requires both `PartialOrd` and `Eq`.
- Trait required for: `gen_range` method from `rand` crate. When storing values in a `BTreeSet<T>` (`Ord`)
- Notes: on structs, compares each field ; on enums, earlier variants are considered less.
##### `Clone` and `Copy` for duplicating values
- What it does: `Clone` to create a deep copy of heap data ; `Copy` to duplicate only the bits on the stack.
- Operators / methods made available: `clone()` method that calls clone on each parts of the type, so each part must implement `Clone` 
- Implementation requirments: `to_vec()` method on a slice
- Trait required: `Clone` is required to implement `Copy`
##### `Hash` for mapping a value to a value of fixed size
- What it does: takes an instance of a type of arbitrary size and map the instance to a value of fixed 
- Operators / methods made available: `hash()` method, called on each part of the `hash()` calling type so each part must implement it
- Trait required for: storing keys in a `HashMap<K, V>` to store data efficiently
##### `Default` for default values
- What it does: creates a default value for a type
- Operators / methods made available: `default()` fun, on each part so each part must implement the trait
- Trait required for: `unwrap_or_default` method on `Option<T>` instances bc if `None` it will return the result of `Default::default()`

## Useful development tools (21.4)
- `$ cargo fmt` of the `rustfmt` tool: auto-formats to standard Rust style (beta, install it via `rustup component add rustfmt` which installs `cargo-fmt` too)
- `cargo fix` of the `rustfix` tool: fixes your code by applying compiler correction suggestions
- `cargo clippy` of the `clippy` linter (beta, install: `rustup component add clippy`): checks code to suggest corrections of common mistakes as `std::f64::consts::PI` instead of `3.1415`
- IDE Integration (VS:Code) using the Rust Language Server: `rustup component add rls rust-analysis rust-src` to gain autocompletion, definition jumps, inline errors etc

## Editions (21.5)
Edition is found in the *Cargo.toml* file
Rust language and compiler are updated every 6 weeks but new *editions* are pushed (in a 6-week update) every 2/3 years into a clear package, with fully updated documentation and tooling. Good and clean and fully updated rally point.
`cargo fix --edition` upgrades code to a new edition