//! §7 — `data class` → derive macros: opt into the freebies you want.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone(); // ~ Kotlin copy() (Clone is the general mechanism)

    assert_eq!(a, b); // PartialEq gives ==
    println!("a == b: {}", a == b);
    println!("debug: {a:?}"); // Debug gives {:?}

    // Struct update syntax ≈ copy(x = 9).
    let c = Point { x: 9, ..a };
    println!("updated: {c:?}");

    // Hash lets it be a HashMap/HashSet key.
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b); // equal to a, so not added twice
    println!("unique points: {}", set.len());

    // Destructuring ≈ componentN.
    let Point { x, y } = c;
    println!("x={x}, y={y}");
}
