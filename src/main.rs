// the following mod / import is required for running the function overflow_error_test further in the code
// mod overflow_error;

fn main() {
    println!("Hello, world!");

    // variables can be defined using the let keyword
    // datatype of the variables can be defined / inferred in rust
    // i32 represents a signed 32 bit number
    // the unsigned equivalent of the datatype would be u32
    // defining the size makes it memory efficient because the size becomes deterministic
    
    // default datatype for numeric variables is i32 as per the video, to confirm later
    let x = 1;
    let y: i32 = 1;

    // floating point types / decimal numbers follow a similar structure
    // the default type inferred for floating point numeric values is f64 by the way
    let z: f32 = 10.001;

    println!("{} = {} != {}", x, y, z);
    // generates the output 1 = 1

    // overflow_error::overflow_error_test();
}
