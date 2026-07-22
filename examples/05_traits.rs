//! §14 — Traits: interface + default methods + extension functions, no inheritance.

// Like a Kotlin interface with a default method.
trait Greet {
    fn name(&self) -> String;
    fn hello(&self) -> String {
        format!("Hello, {}", self.name())
    }
}

struct Dog;
impl Greet for Dog {
    fn name(&self) -> String {
        "Rex".to_string()
    }
}

// Traits double as extension functions — implement one for a type you don't own.
trait Doubler {
    fn double(&self) -> Self;
}
impl Doubler for i32 {
    fn double(&self) -> i32 {
        self * 2
    }
}

fn main() {
    let d = Dog;
    println!("{}", d.hello()); // default method
    println!("21.double() = {}", 21.double()); // "extension" on i32
}
