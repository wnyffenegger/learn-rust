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

### Lifetimes

Problem: a buffer lives in memory some where with an HTTP request. What happens if we de-allocate the buffer while the Request object which points to sections of that buffer is still in use?

This is a dangling pointer issue.

Our `Request` will outlive the buffer.

The Rust compiler forces us to deal with this issue. If your code compiles then there are no dangling references.

But Rust needs help from lifetimes to figure out when
to de-allocate stuff and what slices are pointing to.

#### How to use

Must make your `struct` generic. But not generic over a type but generic over a lifetime.

A lifetime is declared with a single quote inside brackets `<'example_lifetime>`.

#### Lifetimes and types

A lifetime is implicitly part of a type.

Every type has a lifetime implicitly. The only time you intereact with that lifetime is when you need to explicitly modify it from the default behavior.

#### Compiler inferences

The compiler infers that the lifetime of the parameters to a function matches the lifetime
of what is returned by a function.

```rust

fn foo(&[u8]) -> &[u8] {

}

// Is really this under the hood
fn foo(&'a [u8]) -> &'a [u8] {

}

Wherever possible the compiler will infer the lifetime so you do not have to supply it.

#### Example

```rust

// 'buf is the lifetime of the buffer
// which the string slices path and query_string point to
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

// You must declare the lifetime to use it
// in an implementation by adding it to the impl
// keyword
impl<'buf> TryFrom<&[u8]> for Request<'buf> {

}

// But that may not be enough because the buffer
// used by the request needs to pass in the lifetime
// so all variables generated off the buffer get that
// lifetime.

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
        // Since the lifetime is part of the type of
        // the parameter you need to specify it
        // as part of the parameter
        fn try_from(byte_array: &'buf [u8]) -> Result<Self, Self::Error> {
        }
}
```

### Anonymous Functions / Closures

To declare an anonymous function use the pipe syntax.

Anonymous functions are used as parameters to other functions and serve like a closure. Sometimes you need to provide anonymous functions.

The syntax for an anonymous function is to have
all parameters surrounded by pipes `|param1, param2, param3|` and then curly brackets 
```rust

my_funct(|param1, param2, param3| {
    // Anonymous function implementation
})
```

An example from the course:

```rust
 data.entry(key)
    // If the entry already exists 
    // we need to modify it so pass in the function
    // dictating the modification.
    .and_modify(|existing| match existing {
        // Take an existing entry, if single convert to multiple, if multiple add to vec
        Value::Single(prev_val) => {
            *existing = Value::Multiple(vec![prev_val, value]);
        }
        Value::Multiple(vec) => vec.push(value),
    })
    .or_insert(Value::Single(value));

```

### Dereferencing a value

Given a location in memory if you want to change the contents of that location in memory you need to dereference it.

To dereference a location in memory use the `*` syntax.

In the example below we are working with a HashMap where the `existing` variable is a location in memory
in the hashmap. That location contains a pointer which we wish to change to point to a new object (a different pointer).

```rust
 data.entry(key)
    .and_modify(|existing| match existing {
        Value::Single(prev_val) => {
            // Existing points to a location in the HashMap
            // The location contains a Value::Single
            // We wish to replace the location with a Value::Multiple
            // To do that we create a Value::Multiple and dereference existing
            // This only works because all values of an enum are guaranteed to take the same amount of memory
            // If that wasn't the case then Rust would complain
            *existing = Value::Multiple(vec![prev_val, value]);
        }
        Value::Multiple(vec) => vec.push(value),
    })
    .or_insert(Value::Single(value));

```

### Derive Attribute

The derive attribute lets the compiler implement
a trait for you if implementing it yourself
takes too much effort.

```rust
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
```

### Copy & Clone

There are two types of things in Rust: types that live entirely on the stack and types that live on the heap.

Types living on the Stack can be trivially copied, the other type not so much.

For more complex types there is the `Clone` trait
that lets you do a deep copy on the heap.

Anything that has `Copy` implemented must also have `Clone` implemented. This is because if a shallow copy is possible a deep copy is trivial.

```rust
#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    OK = 200,
    BAD_REQUEST = 400,
    NOT_FOUND = 404,
}
```

### Dynamic vs. Static Dispatch (Generics)


#### Dynamic Dispatch

`dyn` tells the compiler to use dynamic dispatch. Basically if you use `Write` then the implementation of the `Write` trait isn't known until runtime. 

Underneath the hood Rust maintains a `vtable` which associates implementations of `Write` with the trait
itself.

At runtime the code will jump to the vtable
and then jump to the implementation. This can get expensive at runtime.

Example
```rust
pub fn send(&self, stream: &mut dyn Write) -> IOResult<()> {
    let body = match &self.body {
        Some(b) => b,
        None => "",
    };

    write!(
        stream,
        "HTTP/1.1 {} {}\r\n\r\n{}",
        self.status_code,
        self.status_code.reason_phrase(),
        body
    )
}
```

#### Static Dispatch

Instead of using `dyn` you can replace it with `impl`. If you do so, then at compile time the compiler will look at all of your usages of that
function.

For every different concrete type the compiler will create a copy of the function with
the concrete type.

The compilation is slower and the binary is larger. But the win for RAM is normally worth it.
