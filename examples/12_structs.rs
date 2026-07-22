//! §6 — Structs & "constructors": data in structs, behavior in impl blocks.

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
}

impl User {
    // Associated function = "constructor" by convention, named `new`.
    fn new(name: String, age: u32) -> Self {
        User { name, age } // field init shorthand
    }

    // &self borrows (reads); &mut self mutates; self consumes.
    fn greet(&self) -> String {
        format!("Hi, I'm {}", self.name)
    }

    fn have_birthday(&mut self) {
        self.age += 1;
    }
}

// Tuple struct and unit struct, for completeness.
struct Point(i32, i32);
struct Marker;

fn main() {
    let mut u = User::new("Ada".into(), 36);
    println!("{}", u.greet());
    u.have_birthday();
    println!("after birthday: {u:?}");

    let p = Point(3, 4);
    println!("point = ({}, {})", p.0, p.1);

    let _m = Marker; // no fields; useful as a type-level flag
}
