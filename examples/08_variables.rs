//! §1 — Variables: immutable-by-default, `mut`, and shadowing.

fn main() {
    let x = 5; // immutable
    println!("x = {x}");

    let mut y = 10; // opt into mutation
    y += 10; // reads then updates the initial value
    println!("y = {y}");

    // Shadowing: re-declare the same name, even with a new type. Not mutation.
    let spaces = "   "; // &str
    let spaces = spaces.len(); // now usize — fine, no `mut`
    println!("spaces count = {spaces}");

    // const is compile-time and needs a type.
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {MAX_POINTS}");
}
