//! §12 — Lifetimes: usually inferred; annotate only when returning a borrow.

// "The returned &str lives as long as both inputs."
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// A struct that holds a reference needs a lifetime too.
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let a = String::from("long string is long");
    let b = "short";
    println!("longest = {}", longest(&a, b));

    let novel = String::from("Call me Ada. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let e = Excerpt {
        part: first_sentence,
    };
    println!("excerpt part = {}", e.part);
}
