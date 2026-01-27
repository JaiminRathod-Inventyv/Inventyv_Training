# Structs and Implementation methods

## Description

This Rust project includes **structs, nested structs, and methods**.

The project includes:

- A `Car` struct with details like brand, model, year, color, engine, and owner.  
- Nested structs: `Engine` (fuel type, mileage) and `Owner` (name, age).  
- Methods to **access and modify data** safely and conveniently.  
- Examples of **instance methods** (`&self`), **mutable methods** (`&mut self`), and **associated functions** (without `self`).  

---

## Concepts Covered

1. **Structs and Nested Structs** 
   - Accessing nested data (`Car.engine.mileage`, `Car.owner.name`).

2. **Getters and Setters**  
   - Individual field access (`get_brand`, `get_mileage`)  
   - Individual field modification (`set_color`, `set_owner_age`)  
   - Combined getters (`get_car_info`)  
   - Combined setters (`set_car_info_with_self`, `set_engine_info`, `set_owner_info`)

4. **Associated Functions**  
   - Creating new instances without needing an existing object (`set_car_info_wo_self`).  
   - Generating information strings without an instance (`get_car_info_with_args`).

---

## Example Usage

```rust
let mut car = Car {
    brand: "BMW".to_string(),
    model: "M3".to_string(),
    year: 2023,
    color: "Black".to_string(),
    engine: Engine { fuel_type: "Diesel".to_string(), mileage: 1500 },
    owner: Owner { name: "Jaimin".to_string(), age: 21 },
};

println!("{}", car.get_car_info());
car.set_color("Red");
car.set_mileage(2000);
