# Rust Training Modules

This directory contains hands-on Rust programming assignments created to understand core Rust concepts such as control flow, structs, ownership, serialization, concurrency, and modular design using Cargo projects.

Each subfolder is a **standalone Rust project** with its own `Cargo.toml`, source code.

---

## Folder Structure

```text
06_RUST/
│
├── _01_Loops/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── README.md
│
├── _02_structs_and_methods/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── README.md
│
├── _03_serde/
│   ├── _01_serde_serialization/
│   │   ├── src/
│   │   │   └── main.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   └── _02_serde_deserialization/
│       ├── src/
│       │   └── main.rs
│       ├── Cargo.toml
│       └── README.md
│
├── _04_ownership_and_borrowing/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── README.md
│
├── _05_request_tracker_assignment/
│   ├── src/
│   │   ├── main.rs
│   │   ├── mutex.rs
│   │   └── rwlock.rs
│   ├── Cargo.toml
│   └── README.md
│
├── _06_modules/
│   ├─ src/
│   │  ├─ main.rs
│   │  └─ modules/
│   │     ├─ _01_loops.rs
│   │     ├─ _02_structs_and_methods.rs
│   │     ├─ _03_serde/
│   │     │  ├─ mod.rs
│   │     │  ├─ _01_serde_serialization.rs
│   │     │  └─ _02_serde_deserialization.rs
│   │     ├─ _04_ownership_and_borrowing.rs
│   │     └─ _05_request_tracker_assignment/
│   │        ├─ main.rs
│   │        ├─ mutex.rs
│   │        └─ rwlock.rs
│   │
│   ├─ Cargo.toml
│   └─ README.md
│
├── _07_hashmap_hashset/
├── src/
│   ├── main.rs
│   ├── hashmap.rs
│   └── hashset.rs
│
├── Cargo.toml
└── README.md
│
├── _08_inventory_management_assignment/
├── src/
│   ├── main.rs
├── Cargo.toml
└── README.md





```

---

## Modules Overview

### _01_Loops

Get hands-on with Rust's core control structures. This module explores how loops (`for`, `while`, `loop`) and conditional statements (`if`, `match`) control program execution.

---

### _02_structs_and_methods

Understand Rust's way of modeling data. This section covers how to define structs, including nested ones, and implement methods with `impl`.

---

### _03_serde

This module focuses on JSON integration in Rust. Using `serde`, how to convert Rust structs into JSON format and parse JSON back into Rust data. It’s split into serialization and deserialization folders to build a clear picture of data interchange.

---

### _04_ownership_and_borrowing

Explore Rust's unique memory management. mutable and immutable references, track how data moves through the program. Setter functions include logging.

---

### _05_request_tracker_assignment

This module introduces a request tracking system where shared data is protected with `Mutex`, and read/write operations use `RwLock`.

---

### _06_modules

Understand Rust's module system and how to organize code for clarity. how to split a project into submodules, re-export functionality using `mod.rs`, and wire everything together from a single entry point (`main.rs`).

---

### _07_hashmap_hashset

This module demonstrates Rust's **HashMap** and **HashSet** collections.\
how to insert, remove, clone, retain, reserve capacity, and extend collections using inbuilt methods of hashmap and hashset in rust.

---

### _08_inventory_management_assignment

This module contains Rust's concept of trait system, generics, error handling, and HashMap collection to store inventory items.

---