//! §13 — Error handling: Result<T, E> and the `?` operator, not exceptions.

use std::num::ParseIntError;

fn main() {
    println!("{:?}", sum_two("2", "40")); // Ok(42)
    println!("{:?}", sum_two("2", "oops")); // Err(...)

    // Handling a Result the explicit way (Kotlin try/catch equivalent):
    match sum_two("10", "5") {
        Ok(n) => println!("got {n}"),
        Err(e) => println!("failed: {e}"),
    }
}

// `?` unwraps Ok or returns the Err immediately — the clean happy path.
fn sum_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}
