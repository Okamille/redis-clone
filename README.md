# Redis Clone in Rust 🦀

A step-by-step implementation of a Redis-like in-memory data store in Rust, designed as a hands-on learning journey through the Rust programming language.

## 📚 Learning Objectives

This project teaches Rust concepts incrementally by building a real-world application:
- Ownership, borrowing, and lifetimes
- Concurrency and thread safety
- Network programming with TCP
- Error handling and Result types
- Data structures and collections
- Pattern matching and enums
- Testing and benchmarking
- Async I/O (optional advanced step)

## 🎯 What We're Building

A simplified Redis implementation supporting:
- Key-value storage (Strings, Lists, Sets, Hashes)
- Basic commands (GET, SET, DEL, EXISTS, etc.)
- TCP server with RESP (Redis Serialization Protocol)
- Command parsing and execution
- Multi-client support with thread pooling
- Optional: Persistence, TTL/expiration, pub/sub

## 📋 Prerequisites

- Rust installed (run `rustup update` to get the latest)
- Basic understanding of programming concepts
- Familiarity with command-line tools
- Optional: Basic knowledge of Redis (helpful but not required)

## 🚀 Quick Start

```bash
# Run the server
cargo run --bin redis-clone

# In another terminal, connect with redis-cli or telnet
redis-cli -p 6379
# or
telnet localhost 6379
```

---

## 📖 Implementation Steps

### **Step 1: Basic TCP Server** ✅
**Concepts**: TCP sockets, basic I/O, Result/Option types

- [ ] Create a TCP listener on port 6379
- [ ] Accept incoming connections
- [ ] Read and echo data back to clients
- [ ] Handle basic errors with `Result<T, E>`

**What You'll Learn**:
- `std::net::TcpListener` and `TcpStream`
- Error handling with `?` operator
- Reading/writing bytes with `BufReader` and `BufWriter`

---

### **Step 2: RESP Protocol Parser**
**Concepts**: Pattern matching, enums, string manipulation

- [ ] Implement RESP data type enum (SimpleString, Error, Integer, BulkString, Array)
- [ ] Parse RESP commands from bytes
- [ ] Serialize responses back to RESP format
- [ ] Handle protocol errors gracefully

**What You'll Learn**:
- Complex `enum` definitions with data
- `match` expressions and pattern destructuring
- Working with `Vec<u8>` and string conversions
- Custom error types with `std::error::Error`

**Example RESP Message**:
```
*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n
→ Array[BulkString("GET"), BulkString("key")]
```

---

### **Step 3: In-Memory Storage**
**Concepts**: HashMap, ownership, borrowing, lifetimes

- [ ] Create a `Store` struct with `HashMap<String, Value>`
- [ ] Implement GET, SET, DEL, EXISTS commands
- [ ] Handle different value types (strings initially)
- [ ] Return proper RESP responses

**What You'll Learn**:
- `std::collections::HashMap`
- Borrowing with `&` and `&mut`
- Ownership transfer vs references
- Method implementations with `impl`

---

### **Step 4: Multi-Client Support**
**Concepts**: Threads, shared state, Arc, Mutex

- [ ] Use thread pool to handle multiple clients
- [ ] Share storage across threads with `Arc<Mutex<Store>>`
- [ ] Handle concurrent reads and writes safely
- [ ] Avoid deadlocks and race conditions

**What You'll Learn**:
- `std::thread` and thread spawning
- `Arc<T>` (Atomic Reference Counting)
- `Mutex<T>` and `RwLock<T>` for interior mutability
- Thread safety and `Send`/`Sync` traits

---

### **Step 5: Command System**
**Concepts**: Trait objects, dynamic dispatch, code organization

- [ ] Define a `Command` trait with `execute()` method
- [ ] Implement separate structs for each command (GetCommand, SetCommand, etc.)
- [ ] Parse raw RESP into command objects
- [ ] Execute commands polymorphically

**What You'll Learn**:
- Trait definitions and implementations
- `Box<dyn Trait>` for dynamic dispatch
- Module system (`mod`, `pub`, `use`)
- Separation of concerns

---

### **Step 6: Complex Data Types**
**Concepts**: Advanced data structures, generics

- [ ] Implement Lists (LPUSH, RPUSH, LPOP, RPOP, LRANGE)
- [ ] Implement Sets (SADD, SREM, SMEMBERS, SISMEMBER)
- [ ] Implement Hashes (HSET, HGET, HDEL, HGETALL)
- [ ] Use appropriate Rust collections for each type

**What You'll Learn**:
- `VecDeque` for lists
- `HashSet` for sets
- Nested `HashMap` for hashes
- Generic programming with `<T>`

---

### **Step 7: Advanced Features**
**Concepts**: Time handling, file I/O, serialization

- [ ] Add key expiration (TTL) with `EXPIRE` command
- [ ] Implement background cleanup of expired keys
- [ ] Add persistence (snapshot to disk)
- [ ] Implement `SAVE` and restore on startup

**What You'll Learn**:
- `std::time::{Instant, Duration}`
- Background threads and channels
- File I/O with `std::fs`
- Serialization (consider `serde` crate)

---

### **Step 8: Testing & Performance**
**Concepts**: Unit tests, integration tests, benchmarking

- [ ] Write unit tests for each command
- [ ] Create integration tests with real TCP connections
- [ ] Add benchmarks to measure throughput
- [ ] Profile and optimize hot paths

**What You'll Learn**:
- `#[cfg(test)]` and `#[test]` attributes
- `assert!`, `assert_eq!` macros
- `cargo bench` and criterion crate
- Performance profiling tools

---

### **Step 9: Error Handling & Logging**
**Concepts**: Custom errors, thiserror, logging

- [ ] Create comprehensive error types
- [ ] Add proper error context and propagation
- [ ] Implement logging with `log` and `env_logger`
- [ ] Add graceful shutdown

**What You'll Learn**:
- `thiserror` and `anyhow` crates
- The `log` facade and logging levels
- Signal handling (SIGINT, SIGTERM)
- RAII and the `Drop` trait

---

### **Step 10 (Optional): Async I/O**
**Concepts**: Async/await, Tokio runtime

- [ ] Convert to async with `tokio`
- [ ] Use `tokio::net::TcpListener`
- [ ] Handle many concurrent connections efficiently
- [ ] Compare performance with threaded version

**What You'll Learn**:
- `async fn` and `.await` syntax
- `Future` trait and executors
- Async mutexes and channels
- When to use async vs threads

---

## 🏗️ Project Structure

```
redis-clone/
├── src/
│   ├── main.rs           # Server entry point
│   ├── lib.rs            # Library root
│   ├── server.rs         # TCP server logic
│   ├── protocol/         # RESP protocol implementation
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   └── serializer.rs
│   ├── storage/          # Data storage layer
│   │   ├── mod.rs
│   │   └── store.rs
│   ├── commands/         # Command implementations
│   │   ├── mod.rs
│   │   ├── get.rs
│   │   ├── set.rs
│   │   └── ...
│   └── types/            # Value types and enums
│       └── mod.rs
├── tests/                # Integration tests
│   └── integration_test.rs
├── benches/              # Benchmarks
│   └── throughput.rs
├── Cargo.toml
└── README.md
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_get_set

# Run benchmarks
cargo bench
```

## 📚 Resources

### Rust Learning
- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises

### Redis & Networking
- [Redis Protocol Specification](https://redis.io/docs/reference/protocol-spec/) - RESP format
- [Redis Commands](https://redis.io/commands/) - Official command reference
- [Build Your Own Redis](https://build-your-own.org/redis/) - Inspiration

### Relevant Crates
- `tokio` - Async runtime (optional)
- `serde` - Serialization framework
- `thiserror` / `anyhow` - Error handling
- `log` / `env_logger` - Logging
- `criterion` - Benchmarking

## 🎓 Learning Tips

1. **Take it slow**: Complete each step fully before moving to the next
2. **Read compiler errors carefully**: Rust's compiler is your teacher
3. **Experiment**: Try breaking things to understand ownership rules
4. **Write tests**: Test-driven development helps learn APIs
5. **Read the docs**: Use `cargo doc --open` to browse documentation
6. **Ask for help**: Rust community is friendly (users.rust-lang.org, Discord)

## 🤝 Contributing

This is a learning project! Feel free to:
- Add new Redis commands
- Improve error messages
- Optimize performance
- Add more tests
- Extend documentation

## 📄 License

MIT License - Learn freely!

---

## 🎯 Current Status

- [x] Project setup
- [ ] Step 1: Basic TCP Server
- [ ] Step 2: RESP Protocol Parser
- [ ] Step 3: In-Memory Storage
- [ ] Step 4: Multi-Client Support
- [ ] Step 5: Command System
- [ ] Step 6: Complex Data Types
- [ ] Step 7: Advanced Features
- [ ] Step 8: Testing & Performance
- [ ] Step 9: Error Handling & Logging
- [ ] Step 10: Async I/O (Optional)

**Next up**: Implement Step 1 - Basic TCP Server

Happy Coding! 🦀✨

