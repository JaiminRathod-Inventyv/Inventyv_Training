# Rust Struct Methods, Ownership, and Borrowing

This project demonstrates fundamental **Rust programming concepts** such as struct design, method implementation, ownership rules, borrowing, and references using a **Car Struct** example. After every setter method call, the program displays both the mutated struct and the original struct state.

The code showcases how Rust handles data safely while allowing controlled modification using mutable references.

---

## Project Description

The program defines a `Car` structure along with nested `Engine` and `Owner` structures.  
It implements multiple getter and setter methods and logs internal state changes whenever a setter is called.

This helps in understanding how `self`, `&self`, and `&mut self` behave in Rust.

---

## Core Concepts Covered

### 🔹 Structs and Nested Structs

- `Car` struct contains basic car details.
- `Engine` struct stores fuel and mileage details.
- `Owner` struct stores owner information.

---

### 🔹 Getter and Setter Methods

- **Getter methods** return field values using immutable borrowing (`&self`).
- **Setter methods** modify fields using mutable borrowing (`&mut self`).

---

## Ownership and Borrowing

- `&self` → Immutable borrow (read-only access)
- `&mut self` → Mutable borrow (allows modification of data)

---

## Running the Program

to compile and run the project, use the following command:

```bash
cargo run
```

## Sample Output

```text
BMW
M3
2023
Black
1500
Diesel
Name: Jaimin, Age: 21
Fuel Type: Diesel, Mileage: 1500

----- after calling set_color -----
using self:
Car { ... }

using &self:
Car { ... }

----- after calling set_mileage -----
using self:
Car { ... }

using &self:
Car { ... }
```