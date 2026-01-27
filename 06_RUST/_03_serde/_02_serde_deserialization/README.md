# Rust Serde Deserialization JSON Example – Car Struct

This project demonstrates how to use the `serde` crate with the `serde_json` crate in Rust to serialize and deserialize a **nested struct (`Car`)** to and from JSON format.

## Overview

The application defines nested data structures (`Car`, `Engine`, and `Owner`) and performs the following operations:

1. **Raw String Handling**: Demonstrates parsing a raw JSON string literal directly into a `Car` struct.
2. **Deserialization**: Parses a JSON string back into a `Car` struct.

---

## Dependencies

The project uses the following dependencies in `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage

To run the project, ensure you have Rust installed and run:

```bash
cargo run
```

## Expected Output

The program will output:
1.  deserialized struct derived from a raw JSON string literal and serialized struct.

```text

Deserialized Car from raw JSON string: Car { brand: "BMW", model: "M3", year: 2023, color: "Black", engine: Engine { fuel_type: "Diesel", mileage: 1500 }, owner: Owner { name: "Jaimin", age: 21 } }

Deserialized Car from JSON: Car { brand: "BMW", model: "M3", year: 2023, color: "Black", engine: Engine { fuel_type: "Diesel", mileage: 1500 }, owner: Owner { name: "Jaimin", age: 21 } }

```
