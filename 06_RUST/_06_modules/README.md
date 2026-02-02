# Rust Modules

This project demonstrates **Rust modules management** using a structured approach. It includes examples of:

- Basic loops
- Structs and methods
- Serde serialization/deserialization
- Mutex and RwLock
- use of `mod.rs`, nested modules, and `pub use`

---

## Project Structure

```text
src/
├─ main.rs                       # root file: calls all modules
├─ modules/
│  ├─ _01_loops.rs               
│  ├─ _02_structs_and_methods.rs 
│  ├─ _03_serde/                 # subfolder with mod.rs
│  │   ├─ mod.rs
│  │   ├─ _01_serde_serialization.rs
│  │   └─ _02_serde_deserialization.rs
│  └─ _05_request_tracker_assignment/ # nested module folder without mod.rs
│      ├─ main.rs
│      ├─ mutex.rs
│      └─ rwlock.rs
```