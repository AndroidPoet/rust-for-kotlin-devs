//! §8–9 — Enums (your sealed class) + match (your when).

#[derive(Debug)]
enum Shape {
    Circle { r: f64 },
    Rect(f64, f64),
    Empty,
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { r } => std::f64::consts::PI * r * r,
            Shape::Rect(w, h) => w * h,
            Shape::Empty => 0.0,
        }
    }
}

fn main() {
    let shapes = [
        Shape::Circle { r: 2.0 },
        Shape::Rect(3.0, 4.0),
        Shape::Empty,
    ];

    for s in &shapes {
        println!("{s:?} → area {:.2}", s.area());
    }

    // match as an expression, with ranges, or-patterns, and a guard.
    for n in [0, 2, 7, 42] {
        let label = match n {
            0 => "zero",
            1 | 2 => "small",
            3..=10 => "medium",
            x if x > 40 => "huge",
            _ => "big",
        };
        println!("{n} → {label}");
    }
}
