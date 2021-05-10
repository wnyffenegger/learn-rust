// Documentation on std https://doc.rust-lang.org/std/index.html
// Documentation on std::io https://doc.rust-lang.org/std/io/index.html

// Equivalent to import
use std::io;


// Main is the Rust entrypoint
fn main() {

    // Needs to be mutable because io library will modify it
    // String is a struct
    let mut input = String::new();

    let s1 = &input;
    let s2 = &input;

    some_fn(input);
    
    io::stdin().read_line(&mut input);

    // Compiler will figure out type on its own
    // All variables default to immutable so you have to explicitly
    // declare as mutable
    let mut mars_weight: f32 = calculate_weight_on_mars(100.0);
    // Will be immutable
    // let mars_weight: f32 = calculate_weight_on_mars(100.0);


    mars_weight = 1000.0 * mars_weight;

    // Macros are used for meta programming
    // A function signature must declare the number and type of parameters
    // but a macro can have variable parameters and types every time it is called

    // Macros are rust code that writes rust code
    // println can take varying arguments and format strings
    println!("Weight on Mars: {}g", mars_weight);
}

// Functions must specify type of each parameter
// arrow indicates return value
fn calculate_weight_on_mars(weight: f32) -> f32 {

    // The last statement in a function that doesn't end
    // in a semi-colon is automatically returned
    (weight / 9.81) * 3.711
}

// Must use a reference '&' to indicate you are borrowing
// the value. This will keep the string from being de-allocated
// when the function completes
fn some_fn(s: String) {
    s.push_str("a")
}

