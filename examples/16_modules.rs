//! §17 — Modules & visibility: private by default (opposite of Kotlin).

mod network {
    pub fn connect() -> String {
        // Can call a private sibling within the same module tree.
        format!("connected via {}", protocol())
    }

    fn protocol() -> &'static str {
        "tcp" // private — not visible outside `network`
    }

    pub mod client {
        pub fn ping() -> &'static str {
            "pong"
        }
    }
}

use network::client;

fn main() {
    println!("{}", network::connect());
    println!("ping -> {}", client::ping());
    // network::protocol();  // ❌ private: won't compile
}
