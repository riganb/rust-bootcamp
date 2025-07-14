pub fn overflow_error_test() {
    // the mut keyword makes the variable explicitly mutable, or for simpler understanding, a variable, rather than act as a constant as far as the primitive datatypes are concerned
    let mut n: i8 = 10;

    // i is prefixed with an underscore because it is not being used, and if not using is it intentional, the rust compiler requires an explicit addition of a prefixed underscore to not warn about it
    for _i in 1..100 {
        n += 100;
    }

    println!("n = {}", n);
}

/**
 * execution of the function above led to this error message during runtime:
 * 
 * thread 'main' panicked at src/overflow_error.rs:6:9:
 * attempt to add with overflow
 * note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
 */
