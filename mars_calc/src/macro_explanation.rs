// Main is the Rust entrypoint
fn main() {

    // Macros are used for meta programming
    // A function signature must declare the number and type of parameters
    // but a macro can have variable parameters and types every time it is called

    // Macros are rust code that writes rust code
    // println can take varying arguments and format strings
    println!("Number: {}, String: {}", 100.0, "test");

    calculate_weight_on_mars(100.0);
}

// Functions must specify type of each parameter
// arrow indicates return value
fn calculate_weight_on_mars(weight: f32) -> f32 {

    // The last statement in a function that doesn't end
    // in a semi-colon is automatically returned
    50.0
}

