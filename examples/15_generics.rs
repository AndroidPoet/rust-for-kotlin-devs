//! §15 — Generics: trait bounds are Kotlin's generic constraints.

// Kotlin: fun <T : Comparable<T>> largest(list: List<T>): T
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

// `where` clause for longer bounds.
fn print_all<T>(items: &[T])
where
    T: std::fmt::Display,
{
    for item in items {
        print!("{item} ");
    }
    println!();
}

// A generic struct.
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: std::fmt::Debug> Pair<T> {
    fn show(&self) {
        println!("({:?}, {:?})", self.first, self.second);
    }
}

fn main() {
    println!("largest int = {}", largest(&[3, 7, 2, 9, 4]));
    println!("largest char = {}", largest(&['a', 'z', 'm']));

    print_all(&["rust", "for", "kotlin", "devs"]);

    let p = Pair {
        first: 1,
        second: 2,
    };
    p.show();
}
