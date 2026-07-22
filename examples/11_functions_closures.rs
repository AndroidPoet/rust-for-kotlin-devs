//! §4 — Functions & closures: blocks are expressions.

fn add(a: i32, b: i32) -> i32 {
    a + b // no `;`, no `return` — last expression is the value
}

fn main() {
    println!("add = {}", add(2, 40));

    // A block is an expression: the last line without `;` is its value.
    let y = {
        let x = 3;
        x + 1 // block evaluates to 4
    };
    println!("block value = {y}");

    // Closures (lambdas).
    let mul = |a: i32, b: i32| a * b;
    println!("closure = {}", mul(6, 7));

    let doubled: Vec<i32> = (1..=5).map(|x| x * 2).collect();
    println!("doubled = {doubled:?}");

    // Closures capture their environment.
    let factor = 10;
    let scale = |x: i32| x * factor;
    println!("scaled = {}", scale(5));
}
