# Rust Serde JSON Examples – Serialization & Deserialization

This repository demonstrates how to use the **`serde`** and **`serde_json`** crates in Rust to **serialize and deserialize nested structs** to and from JSON format.

It contains two focused examples using a common `Car` data model with nested structures like `Engine` and `Owner`.

---

## 📁 Project Structure

```text
_03_serde/
├── 01_serde_serialization/
│   ├── src/
│   │   └── main.rs
│   └── README.md
│
└── 02_serde_deserialization/
    ├── src/
    │   └── main.rs
    └── README.md
```

## Data Model Overview

Both examples use the same nested Rust structs:

- `Car`
  - `Engine`
  - `Owner`

This setup helps demonstrate how **nested Rust structures** are handled during JSON serialization and deserialization.

---
## `01_serde_serialization/`

### Features
- Converts a Rust Car struct into a JSON string
- Uses #[derive(Serialize)]
- Serializes nested structs with serde_json

---
## `02_serde_deserialization/`

### Features
 - Parses a raw JSON string literal
 - Deserializes JSON back into a Rust struct
 - Uses #[derive(Deserialize)]
 - Handles nested JSON objects

---