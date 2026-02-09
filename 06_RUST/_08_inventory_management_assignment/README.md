# Generic Inventory System in Rust

A type-safe, generic inventory management system demonstrating Rust's trait system, generics, error handling, and HashMap.

## Problem Statement

Create a generic `Inventory<T>` struct that manages a collection of items (using `Vec<T>` or `HashMap<String, T>`). Define a `DisplayItem` trait that types must implement to show their details.

**Requirements:**
- Use `HashMap<String, T>` to store items by ID, where `T: DisplayItem + Clone`
- Implement error handling with a custom `InventoryError` enum for duplicate IDs, invalid IDs, or missing items
- Make `Inventory<T>` generic over `T` where `T: DisplayItem + Clone`
- The `DisplayItem` trait requires a `display(&self) -> String` method

**Functions to Implement:**
- `add_item(&mut self, id: String, item: T) -> Result<(), InventoryError>` – Add with duplicate check
- `display_all(&self) -> String` – Iterate collection and use trait to show formatted details

## Features

- Generic type support with `DisplayItem + Clone` trait bounds
- Custom error handling with `InventoryError` enum
- Duplicate ID prevention and validation

## Core Components

### `DisplayItem` Trait
```rust
trait DisplayItem {
    fn display(&self) -> String;
}
```

### `Inventory<T>` Struct
```rust
struct Inventory<T> {
    items: HashMap<String, T>,
}
```

### `InventoryError` Enum
```rust
enum InventoryError {
    ItemNotFound(String),
    DuplicateItem(String),
    InvalidId,
}
```

## Usage Examples

### String Inventory
```rust
let mut inventory = Inventory::<String>::new();
inventory.add_items("I01".to_string(), "item1".to_string())?;
println!("{}", inventory.display_all());
```


### Run the Program

```bash
cargo run
```

## Output

```
-------- Inventory T as String-------
-----DISPLAYING INVENTORY-------
inventory is empty!!
item added successfully!
item added successfully!
item added successfully!
provided item-id is invalid
item found :
item1
-----DISPLAYING INVENTORY-------
I01  :  item1
I03  :  item3
I02  :  item2
-------- Inventory T as Product Struct-------
product added!
Product added!
Error: item already exists with ID : p02
Error: provided item-id is invalid
item found :
Product { product_id: "p01", product_name: "Laptop" }
-----DISPLAYING INVENTORY-------
p01  :  Product { product_id: "p01", product_name: "Laptop" }
p02  :  Product { product_id: "p02", product_name: "Phone" }
```