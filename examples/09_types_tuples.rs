//! §2 — Built-in types & tuples.

fn main() {
    let sum: i64 = 1_000_000 * 2;
    println!("sum = {sum}");

    // Explicit widths; usize is the index/length type.
    let byte: u8 = 255;
    let idx: usize = 3;
    println!("byte = {byte}, idx = {idx}");

    // char is a full Unicode scalar (4 bytes), not a UTF-16 unit.
    let heart: char = '♥';
    println!("char = {heart}");

    // Tuples ≈ Pair/Triple.
    let tuple: (i32, f64, char) = (500, 6.4, 'z');
    let (a, b, c) = tuple; // destructuring
    println!("destructured: {a}, {b}, {c}");
    println!("by index: {}", tuple.0);

    // Overflow is explicit when you mean it.
    let wrapped = 250u8.wrapping_add(10); // 4, not a panic
    println!("wrapping_add = {wrapped}");
    println!("checked_add = {:?}", 250u8.checked_add(10)); // None
}
