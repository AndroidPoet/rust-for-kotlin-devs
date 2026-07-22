//! §16 — Collections & iterator chains (very close to Kotlin's).

use std::collections::HashMap;

fn main() {
    // Kotlin: (1..10).filter { it % 2 == 0 }.map { it * it }
    let evens_squared: Vec<i32> = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("{evens_squared:?}");

    // fold / sum / find
    let total: i32 = (1..=5).sum();
    println!("sum 1..=5 = {total}");

    let first_big = (1..=100).find(|&x| x > 42);
    println!("first > 42 = {first_big:?}"); // Some(43)

    // A HashMap (your Map), built by grouping word lengths.
    let words = ["rust", "is", "fun", "and", "safe"];
    let mut by_len: HashMap<usize, Vec<&str>> = HashMap::new();
    for w in words {
        by_len.entry(w.len()).or_default().push(w);
    }
    let mut lens: Vec<_> = by_len.keys().copied().collect();
    lens.sort();
    for len in lens {
        println!("len {len}: {:?}", by_len[&len]);
    }
}
