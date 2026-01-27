# Rust Serde Serialization JSON Example – Car Struct

This project demonstrates how to use the `serde` crate with the `serde_json` crate in Rust to serialize and deserialize a **nested struct (`Car`)** to JSON format.

## Overview

The application defines nested data structures (`Car`, `Engine`, and `Owner`) and performs the following operations:

1. **Serialization**: Converts a Rust `Car` instance into a JSON string.

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

- The serialized JSON string derived from a struct.

```text
Serialized Car : {"brand":"BMW","model":"M3","year":2023,"color":"Black","engine":{"fuel_type":"Diesel","mileage":1500},"owner":{"name":"Jaimin","age":21}}

```
