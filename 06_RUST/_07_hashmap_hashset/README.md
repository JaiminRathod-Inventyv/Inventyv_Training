# Rust HashMap and HashSet Example

This project demonstrates how to use **HashMap** and **HashSet** in Rust with custom structs.\
It covers common operations such as inserting, cloning, reserving capacity, removing elements, retaining elements, and extending
collections.

---

## Project Structure

    src/
    ├── main.rs
    ├── hashmap.rs
    └── hashset.rs

---

## Car Struct

Both examples use a `Car` struct:

``` rust
#[derive(Debug, Clone)]
struct Car {
    car_id: u32,
    brand: String,
}
```

For `HashSet`, additional traits are required:

``` rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
```

These traits are necessary so Rust can compare and hash the struct values.

---

## HashMap

The `hashmap.rs` module demonstrates:

-   Creating a `HashMap<u32, Car>`
-   Inserting key-value pairs
-   Cloning the HashMap
-   Reserving capacity using `try_reserve`
-   Removing entries using `remove`
-   Filtering entries using `retain`
-   Extending the HashMap using `extend`

---

## HashSet

The `hashset.rs` module demonstrates:

-   Creating a `HashSet<Car>`
-   Inserting unique elements
-   Cloning the HashSet
-   Reserving capacity using `try_reserve`
-   Removing elements using `take`
-   Filtering elements using `retain`
-   Extending the HashSet using `extend`

---

## How to Run

Make sure you have Rust installed. Then run:

``` bash
cargo run
```

---

## Key Concepts Covered

-   HashMap vs HashSet
-   Capacity management with `try_reserve`
-   Retaining filtered data
-   Extending collections

---

## Notes

-   `HashMap<K, V>` stores **key-value pairs**.
-   `HashSet<T>` stores **unique values only** (no duplicates).
-   `HashSet` requires `PartialEq`, `Eq`, and `Hash` traits.
-   `clone()` creates a deep copy of the collection.

---