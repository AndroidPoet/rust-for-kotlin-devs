//! §3 — Strings: the #1 gotcha — `String` (owned) vs `&str` (borrowed).

fn main() {
    let literal: &str = "hello"; // literals are &str
    let mut owned: String = String::from("hello"); // owned, growable
    owned.push_str(", world");
    println!("owned = {owned}");

    // Take &str as a parameter — it accepts both String and &str.
    println!("greet: {}", greet(&owned)); // deref coercion String -> &str
    println!("greet: {}", greet(literal));

    // Interpolation.
    let name = "Ada";
    let age = 36;
    println!("{name} is {age}"); // inline capture
    let msg = format!("{name} is {age}"); // returns a String
    println!("formatted = {msg}");

    // No integer indexing (UTF-8 is ambiguous). Iterate instead.
    for ch in "héllo".chars() {
        print!("[{ch}]");
    }
    println!();
    println!("byte length of 'héllo' = {}", "héllo".len()); // 6, not 5
}

fn greet(who: &str) -> String {
    format!("Hi, {who}")
}
