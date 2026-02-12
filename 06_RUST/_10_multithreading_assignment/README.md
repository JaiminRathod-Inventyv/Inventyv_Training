# Multithreaded Rust Application

A concurrent Rust application demonstrating thread synchronization, shared memory management, and atomic operations using 6 concurrent threads operating on shared data.

## Problem Statement

Create a multithreaded Rust application that runs 6 concurrent threads working on shared data.
 
### Data Structure
 
Define a common structure:
```rust
struct MultiThread {
    id: i32,
    recordAddedTime: String,
    threadId: String, // randomly generated id
}
```
 
Maintain a global counter that generates unique id values for new records. All threads will operate on a shared in-memory collection of MultiThread.
 
### Thread Responsibilities

#### Thread 1 — Record Creator
- Add a new MultiThread record every 10 seconds.
- Use the global counter to assign a unique id.
 
#### Thread 2 — State Printer
- Continuously print the current state of all records stored in memory.
 
#### Thread 3 — Even Record Cleaner
- Remove records that:
  - Have an even id, AND
  - Were added more than 20 seconds ago.
 
#### Thread 4 — Odd Record Cleaner
- Remove records that:
  - Have an odd id, AND
  - Were added more than 20 seconds ago.
 
#### Thread 5 — Even Counter
- Print the total number of records with even ids.
 
#### Thread 6 — Odd Counter
- Print the total number of records with odd ids.


## Implementation

1. **Arc**: Enables multiple ownership of shared data across threads
2. **RwLock**: Allows multiple readers or a single writer
3. **AtomicI32**: Thread-safe atomic integer for the global counter
4. **Thread Spawning**: Native Rust threading with `std::thread`

## Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
chrono = "0.4"
```

## Usage

Run the application:

```bash
cargo run
```

The application will start all 6 threads and run indefinitely.

## How It Works

1. **Initialization**: Creates a shared vector wrapped in `Arc<RwLock<>>` and clones it for each thread
2. **Thread 1**: Increments the atomic counter, creates a new record with timestamp and thread ID, then adds it to the shared collection
3. **Thread 2**: Acquires a read lock and prints all current records
4. **Threads 3 & 4**: Acquire write locks, filter records based on age (>20 seconds) and ID (even or odd)
5. **Threads 5 & 6**: Acquire read locks and count records with even/odd IDs respectively

## Synchronization Details

- **Write Operations** (Threads 1, 3, 4): Acquire exclusive write lock using `.write().unwrap()`
- **Read Operations** (Threads 2, 5, 6): Acquire shared read lock using `.read().unwrap()`
- **Lock Scoping**: Locks are released when goes out of scope (end of code block)