# HTTP Server


## Components

1. TCP Listener
1. Http Parser
1. Interface

## Design

1. Want to instantiate the server with the configuration
1. Want the server to start and run forever

```rust
    let server = Server::new("127.0.0.1:8080")
    server.run()
```

# Concepts

## Structs

Is a concept like an object in Rust.

Unlike an object a struct has a definition and an implementation.

The definition contains the fields, the implementation contains actual functionality.

```rust

// Definition
struct Sever {
    addr: string
}

// Implementation block
impl Server {

}

```

### Method

There are two types of functionality associated with a struct.

**Methods are like instance methods in Java for objects**

Methods always take a `self` argument.

```rust
impl Server {
    // Careful, the struct will be de-allocated due to memory rules
    fn run(self) {

    }

    // Struct will exist after function exits
    fn run(&self) {

    }

    // Struct is mutable
    fn rn(mut &self) {

    }
}
```

### Functions

Associated functions are the second type. They are more like static methods in Java.

An example of one such functions is new

```rust
impl Server {
    fn new(addr: String) -> Server {
        Server {
            addr: addr
        }
    }
}
```

### Constructor Standards

The standard is to use `new` for all constructors but it is not a requirement.

```rust
impl Server {
    // Self is syntactic sugar for the Server struct
    fn new(addr: String) -> Self {
        Self {
            addr: addr
        }
    }
}
```

## Strings

### Strings in Memory

Stack Component:
* Length of the string
* Capacity of buffer on heap
* Pointer to Heap

Heap:
* String in buffer

**If string length exceeds buffer then the string moves in the heap**

#### String Slice

A string slice is an immutable reference to a part of a string.

It is a range of values from a string.

String slices are immutable.

```rust
    // Normal string
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..14];

    // Converts string to string slice pointing to original string
    let string_borrow: &str = &string;
```

### String Slice Mechanics

Stack:
* Length
* Pointer to 

When a slice is done the string slice **points to the
location in the original string where the slice begins**

### String Slice Indexes

* Using plain indexes `10..14` is dangerous because the index is really a byte not a character.
* Characters are encoded in UTF-8.
* `10..14` means give me everything after the 10th byte.

* If the index is not at the end or beginnning of a character Rust will panic and exit.

### Compiler and Strings vs. String Slices

Strings and string slices look the same as method parameters and the compiler can auto-convert between the two.

## Enums

In memory enum variants are represented by numbers
0, 1, 2, etc.

Enums are like C Unions on steroids.

Each variant of an enum can contain a different type. But when you do this every variant of an enum will take up enough memory to contain the largest type of the variant.

Every variant from the below enum will be allocated enough space to contain a string.

```rust
enum Method {
    GET(String),
    DELETE(u64)
}
```

### Option Enum (Null in Rust)

Rust does not support null but needs a way to express None in a type safe manner.

The Option enum is that tool:

* None: no value
* Some: some value of the generic enum type

Option is always in scope in every Rust file.

```rust
/// The `Option` type. See [the module level documentation](self) for more.
pub enum Option<T> {
    /// No value
    None,
    /// Some value `T`
    Some(T),
}
```

### Modules

Organize code with modules. Every file in rust is treated as a module. Submodules can be created using the `mod` keyword.

In a module the default scope is private. All definitions, implementations, and even submodules default to private.

Use the pub keyword to make it public.

For a nested module use the super keyword to step up a level.

```rust

// use will pull specific parts of module in
use server::Server;

// Compiler will go find server module and pull it into scope
mod server;
```

Some facts about modules:

* Modules can be created using the `mod` keyword
* Modules can be imported using the `mod` keyword
* Everything in a module is private by default
* Everything in a module can be made public using the `pub` keyword
* Every file in Rust is considered a module
* Every directory in Rust is considered a module
* A parent module can be referred to using `super`
* Sub modules must be made public using the `pub` keyword

### mod.rs

In a directory you need to provide a `mod.rs` file which works like a Python `__init__.py` file.

At a minimum it needs to include 
`pub mod submodule.rs` for all submodules. 

But you can manage the exposure of specific structs using: `pub use submodule::SubStruct;`


### Result Enum (Rust Error Handling)

Errors happen. This is how Rust handles them.

**Rust requires you acknowledge the possibility of an error before compiling your code**

Categories of Errors:

* Recoverable: errors that can be handled, file not found
* Unrecoverable: errors that cannot be handled, index out of bounds

Rust's equivalent to exceptions is `Result`. Like the `Option` enum, this enum is included by default in every scope.

```rust
/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
///
/// See the [module documentation](self) for details.
pub enum Result<T, E> {
    /// Contains the success value
    Ok(T),

    /// Contains the error value
    Err(E),
}
```

The `Result` enum can be extended to make custom module result types.

```rust
    pub type Result<T> = result::Result<T, Error>;
```

**A recoverable error can be converted to an unrecoverable error by calling Result::unwrap**

```rust
    pub fn unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
        }
    }
```

### Loops

Rust treats infinite loops differently then executable loops.

* `loop` is an infinite loop
* `break` exits a loop
* `continue` skips an iteration

Break in rust is more powerful 
```rust
    'outer: loop {
        loop {
            // Will break from outer loop
            break 'outer;

            // Will skip iteration
            continue 'outer;
        }
    }
```

### Tuples

Rust uses tuples to group values.

Tuples in Rust have a fixed length and each element
in the tuple has a defined type.

Syntax for a tuple is a comma separated list with parenthesis.

```rust
    let tup = (5, "a", listener); 

    // Can assign values in a tuple to a value
    let (stream, addr) = res.unwrap();
```

### Match (kind of like switch)

Does a value match a pattern?

For enums won't compile unless all values match an enum. Match works on pretty much anything.

* Enums
* Strings


Syntax:
```rust
    match listener.accept() {
        // Underscore in this case will ignore addr
        Ok((stream, _)) => {
            
        },
        // Use underscore to ignore values
        Err(_) => println("Err");
    }

    match "abcd" {
        // Matches string
        "abcd" => println!(),
        // Matches a or b
        "a" | "b" => {},
        // Default case
        _ => {}
    }
```

### Arrays

Compiler needs to know the size of the array to pass it in so it knows the size of the stack frame.

```rust
// Dumb way
fn arr(a: [u8; 5]) {}

// Smart way
fn arr(a: &[u8])

let a = [1, 3, 43, 2, 3]

arr(&a);
arr(&a[1..3])
```

But there is a way around this: pass a reference
to an array. When you pass a reference to an array
you are actually passing a `slice` of the array.

Rust forces each index in the array has to have a value

```rust
let mut buffer = [0; 1024];
```

### Traits

A trait is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait.

Traits can be implemented for any data type. In the example below, we define Animal, a group of methods. The Animal trait is then implemented for the Sheep data type, allowing the use of methods from Animal with a Sheep.


```rust

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}
```

**Traits can be used to define methods on already existing types**

Example:

```rust

trait Encrypt {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

// We are defining new methods on already existing types. These new methods are now usable across our
// entire crate and even by other crates.

impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

impl Enrypt for &[u8] {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}
```

**Traits can implement other traits**

```rust
pub trait Error: Debug + Display {
    ...
}
```

### TryFrom

Reflexively implemented so if you implement

```rust
impl TryFrom<&[u8]> for Request
```

Then you get the following functionality

```rust
// Convert a bu
Request::try_from(&buffer[..])

// And it sticks the method as a try_into onto the byte slice type
Result<Request, _> = &buffer[..].try_into();
```

### Crate Keyword

The crate keyword refers to the root of the project/crate that you are currently working on.

### Debug & Display Trait

Traits for standardizing logging. Anything with these methods completed can automatically be
logged to the command line.

```rust
// Use display trait in println
println!("{:?}", foo);

// Implement debug and display trait
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
```

### Iterators

Rust iterators have a single method `next` which returns an `Option<_>`.

Rust `for` loops can iterate over an iterator.

```rust

// Contains just a character
for c in request.chars() {
    ...
}

// Contains both a character and an index
for (i, c) in request.chars().enumerate() {

}
```

### Convert Option -> Result

Sometimes Option.None really means there is an error. When that is the case use the same syntax as
used for advanced error handling.

The `ok_or` method says either return Option.Some as an ok *or* return the specified error. The `?` effectively unwraps the result.

Example:

```rust

option.ok_or(ParseError::InvalidRequest)?;
```