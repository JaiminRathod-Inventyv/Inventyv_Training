# HTTP Server Assignment

A multi-threaded HTTP server built with Rust and Axum that provides a RESTful API for managing owners and their cars. The server uses JSON file-based persistence and supports concurrent requests across multiple threads.

## Features

- **Multi-threaded Architecture**: Handles concurrent requests using Tokio's multi-threaded runtime (4 worker threads)
- **RESTful API**: Complete CRUD operations for owners and cars
- **JSON Persistence**: Data is automatically saved to `owners.json` file
- **Thread-safe Operations**: Uses `Arc<RwLock>` for safe concurrent access to shared state
- **UUID Generation**: Automatic ID generation for owners and cars


## Project Structure

```
_11_http_server_assignment/
├── src/
│   ├── main.rs          # Server entry point and configuration
│   ├── api.rs           # API endpoint handlers
│   ├── routes.rs        # Route definitions
│   ├── handler.rs       # Data saving utilities
│   └── model.rs         # Data models (Owner, Car)
├── owners.json          # JSON database file
├── Cargo.toml           # Project dependencies
└── README.md           
```

## Architecture

The server uses a shared state pattern with `Arc<RwLock<Vec<Owner>>>` to manage in-memory data. All operations are logged with thread IDs to demonstrate multi-threading capabilities.

### Data Models

**Owner:**
```rust
{
    id: String,
    name: String,
    email: String,
    cars: Vec<Car>
}
```

**Car:**
```rust
{
    id: String,
    name: String,
    model: String,
    year: i32,
    registration_number: String
}
```

## API Endpoints

**Owners:**
- `GET /owners` - Get all owners
- `GET /owners/{id}` - Get owner by ID
- `POST /owners` - Create owner
- `PUT /owners/{id}` - Update owner
- `DELETE /owners/{id}` - Delete owner

**Cars:**
- `GET /cars` - Get all cars
- `GET /cars/{id}` - Get car by ID
- `POST /owners/{owner_id}/cars` - Add car to owner
- `PUT /owners/{owner_id}/cars/{car_id}` - Update car
- `DELETE /owners/{owner_id}/cars/{car_id}` - Delete car

## Getting Started

**Run:**
```bash
cargo run
```
Server starts at `http://127.0.0.1:8080`

---
