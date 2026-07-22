//! §18 — Async: `async`/`.await` + a runtime (tokio), the coroutine analog.
//!
//! Run with: cargo run --example 07_async

use std::time::Duration;

async fn fetch(label: &str, ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    format!("{label} done")
}

#[tokio::main]
async fn main() {
    // Run two futures concurrently, like awaitAll / coroutineScope.
    let (a, b) = tokio::join!(fetch("a", 30), fetch("b", 10));
    println!("{a}");
    println!("{b}");

    // A Future does nothing until awaited (lazy, unlike a launched coroutine).
    let pending = fetch("c", 5); // not running yet
    let c = pending.await; // now it runs
    println!("{c}");
}
