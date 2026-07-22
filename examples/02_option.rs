//! §5 — No null: Option<T> replaces nullable types.

fn main() {
    let name = lookup("ada"); // Some("Ada")
    let missing = lookup("bob"); // None

    // Kotlin: name?.length
    let len: Option<usize> = name.as_ref().map(|n| n.len());
    println!("mapped length: {len:?}");

    // Kotlin: missing ?: "default"
    let value = missing.unwrap_or_else(|| "default".to_string());
    println!("fallback: {value}");

    // Kotlin: if (name != null) { ... }  (smart cast)
    if let Some(n) = &name {
        println!("have a name: {n}");
    }

    // Kotlin: val n = name ?: return   →   let-else
    let Some(n) = &name else {
        println!("no name; bailing");
        return;
    };
    println!("let-else bound: {n}");

    // `?` on Option inside a function that returns Option:
    println!("first char: {:?}", first_char("rust"));
    println!("first char of empty: {:?}", first_char(""));
}

fn first_char(s: &str) -> Option<char> {
    let c = s.chars().next()?; // returns None early if empty
    Some(c)
}

// A tiny "database" so the compiler can't see the None at compile time.
fn lookup(key: &str) -> Option<String> {
    match key {
        "ada" => Some("Ada".to_string()),
        _ => None,
    }
}
