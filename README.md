# Learn Rust

Code and notes from interactive Rust course.

[Udemy Course](https://www.udemy.com/course/rust-fundamentals/)

# Notes

## Dependency and Package Management

cargo new <project-name>

cargo new --help

Every rust project has a cargo.toml file at the root of the project

### Cargo file

The cargo.toml file contains

* Metadata about the project
* Dependencies of the project

### Crates

* Crates registry: https://crates.io/


### Build

cargo build

### Cargo Expand

* instructions: https://crates.io/crates/cargo-expand
* install `cargo expand`


## Ownership

Three rules:

1. Each value in Rust is owned by a variable
1. When the owner goes out of scope the value will be de-allocated (this is called RAII).
1. There can only be one owner at a time

### Example

```rust
    // Simple types
    // Size is known
    // and value 5 is copied to new location.
    // We have two 5s and not two pointers
    let a = 5;
    let b = a;

    // Complex types
    // cannot 'move' value of string to another variable since it cannot be copied
    let mut input = String::new();

    // Will not compile because only one owner allowed
    // This will lead to a double free
    let mut s = input;
    io::stdin().read_line(&mut input);
```

1. a string is created on the heap
1. the string is owned by the input variable which is on the stack (1)
1. when input goes out of scope the string will be de-allocated (2)
1. rust compiler will run the drop function (destroy) when input goes out of scope. This is called RAII (2)
1. attempting to build the code causes an error (3)
1. strings cannot be copied because their size is not
pre-determined on the heap. Only standard size types
can be copied (3)
1. you have to explicitly `borrow` via `references` to get around the issue (3)

Compiler error caused by running this code:

```
error[E0382]: borrow of moved value: `input`
  --> src/main.rs:15:27
   |
13 |     let mut input = String::new();
   |         --------- move occurs because `input` has type `String`, which does not implement the `Copy` trait
14 |     let mut s = input;
   |                 ----- value moved here
15 |     io::stdin().read_line(&mut input);
   |                           ^^^^^^^^^^ value borrowed here after move

error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0382`.
error: could not compile `mars_calc`
```


```rust
    // Complex types
    // cannot 'move' value of string to another variable since it cannot be copied
    let mut input = String::new();

    // some_fn becomes the owner of the input
    // when it goes out of scope it will be de-allocated
    some_fn(input);

    io::stdin().read_line(&mut input);

    fn some_fn(s: String) {}
```

1. some_fn becomes the owner of input
1. by rule (2) the variable will be de-allocated
1. it is out of scope by the time readline is called

```
error[E0308]: mismatched types
  --> src/main.rs:14:13
   |
14 |     some_fn(input);
   |             ^^^^^
   |             |
   |             expected `&String`, found struct `String`
   |             help: consider borrowing here: `&input`

error: aborting due to previous error
```

## Borrowing

You can borrow by passing a reference to a variable

* Values passed by reference are immutable by default
* You have to mark a reference as mutable

```rust
   // Valid
   fn some_fn(s: &mut String) {
      s.push_str("a")
   }

   // Invalid
   fn some_fn(s: &String) {
      s.push_str("a")
   }
```

Compiler Error:
```
error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
  --> src/main.rs:49:5
   |
48 | fn some_fn(s: &String) {
   |               ------- help: consider changing this to be a mutable reference: `&mut String`
49 |     s.push_str("a")
   |     ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

Mutable vs. Immutable References

For a single object in the same scope you can have either:
* As many immutable references as you want or
* Only one mutable reference

Rust prevents you from mixing immutable and mutable references to objects.
* As many immutable references as you want
* One mutable reference
* You cannot have two mutable references
* 

```rust
// Legal
let s1 = &input;
let s2 = &input;

// Illegal
let mut s1 = &mut input;
let s2 = &input;

// Legal
let mut s1 = &mut input

// Illegal
let mut s1 = &mut input;
let mut s2 = &input;
```