# Async Connection Pool

A bounded async connection pool built in Rust with a guard pattern — 
because managing shared resources safely across concurrent tasks is one 
of those problems that looks simple until it isn't.

The idea is straightforward: you have a fixed number of connections and 
many tasks competing for them. When a task gets a connection it holds it 
exclusively, does its work, and the connection automatically returns to 
the pool when it goes out of scope. No manual release, no forgotten returns, 
no leaks — Rust's `Drop` trait handles it.

What made this interesting to build is that the same pattern shows up 
directly in ML inference systems. GPU KV cache slots work exactly like 
connections in a pool — fixed capacity, multiple requests competing for 
slots, slots must be released after each request completes. Building this 
in Rust made that mental model concrete.

## What it covers

- `Semaphore` for bounding concurrent access
- RAII guard pattern with `Deref` and `Drop`
- Why `std::sync::Mutex` and not `tokio::sync::Mutex` — `Drop` is 
  synchronous and cannot `.await`
- `Arc` for shared ownership across async tasks

## Run it

```bash
cargo test
```

