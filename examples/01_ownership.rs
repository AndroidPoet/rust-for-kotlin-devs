//! §10–11 — Ownership & borrowing: the concept with no Kotlin equivalent.

fn main() {
    // MOVE: heap types transfer ownership on assignment.
    let s1 = String::from("hi");
    let s2 = s1; // ownership moves to s2; s1 is now invalid.
    // println!("{s1}"); // ← uncomment to see the compile error.
    println!("moved into s2: {s2}");

    // COPY: small stack types are copied, not moved.
    let a = 5;
    let b = a; // both valid.
    println!("copied: a={a}, b={b}");

    // BORROW: lend a read-only view without giving up ownership.
    let owner = String::from("borrow me");
    let length = len(&owner);
    println!("'{owner}' has length {length}"); // owner still usable.

    // MUTABLE BORROW: exactly one at a time.
    let mut greeting = String::from("hello");
    shout(&mut greeting);
    println!("after &mut: {greeting}");

    // The aliasing-XOR-mutability rule, demonstrated safely:
    let mut v = vec![1, 2, 3];
    {
        let first = &v[0]; // shared borrow starts
        println!("first is {first}"); // ...and ends here (last use)
    }
    v.push(4); // now allowed — no live borrow
    println!("vec is now {v:?}");
}

fn len(s: &str) -> usize {
    s.len()
}

fn shout(s: &mut String) {
    s.push('!');
}
