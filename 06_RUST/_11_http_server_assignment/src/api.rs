use crate::{SharedState, handler::save_owners, model::*};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use uuid::Uuid;

use std::thread;

/// Get all owners
pub async fn get_owners(State(state): State<SharedState>) -> Json<Vec<Owner>> {
    let thread_id = thread::current().id();
    println!("get_owners request on thread: {:?}", thread_id);
    let owners = state.read().await;
    Json(owners.clone())
}

/// Get single owner
pub async fn get_owner_by_id(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Owner>, StatusCode> {
    let thread_id = thread::current().id();
    println!("get_owner_by_id request on thread: {:?}", thread_id);
    let owners = state.read().await;

    owners
        .iter()
        .find(|o| o.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Create owner
pub async fn create_owner(
    State(state): State<SharedState>,
    Json(mut owner): Json<Owner>,
) -> (StatusCode, Json<Owner>) {
    let thread_id = thread::current().id();
    println!("create_owner request on thread: {:?}", thread_id);
    owner.id = Uuid::new_v4().to_string();
    owner.cars = vec![];

    let mut owners = state.write().await;
    owners.push(owner.clone());

    save_owners(&owners).await;

    (StatusCode::CREATED, Json(owner))
}

/// Update owner
pub async fn update_owner(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(updated): Json<Owner>,
) -> StatusCode {
    let thread_id = thread::current().id();
    println!("update_owner request on thread: {:?}", thread_id);
    let mut owners = state.write().await;

    if let Some(owner) = owners.iter_mut().find(|o| o.id == id) {
        owner.name = updated.name;
        owner.email = updated.email;
        owner.cars = updated.cars;

        save_owners(&owners).await;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Delete owner
pub async fn delete_owner(Path(id): Path<String>, State(state): State<SharedState>) -> StatusCode {
    let thread_id = thread::current().id();
    println!("delete_owner request on thread: {:?}", thread_id);
    let mut owners = state.write().await;

    let len_before = owners.len();
    owners.retain(|o| o.id != id);

    if owners.len() != len_before {
        save_owners(&owners).await;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Add car to owner
pub async fn add_car(
    Path(owner_id): Path<String>,
    State(state): State<SharedState>,
    Json(mut car): Json<Car>,
) -> Result<(StatusCode, Json<Car>), StatusCode> {
    let thread_id = thread::current().id();
    println!("add_car request on thread: {:?}", thread_id);
    car.id = Uuid::new_v4().to_string();

    let mut owners = state.write().await;

    if let Some(owner) = owners.iter_mut().find(|o| o.id == owner_id) {
        owner.cars.push(car.clone());

        save_owners(&owners).await;

        Ok((StatusCode::CREATED, Json(car)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Update car
pub async fn update_car(
    Path((owner_id, car_id)): Path<(String, String)>,
    State(state): State<SharedState>,
    Json(updated): Json<Car>,
) -> StatusCode {
    let thread_id = thread::current().id();
    println!("update_car request on thread: {:?}", thread_id);
    let mut owners = state.write().await;

    if let Some(owner) = owners.iter_mut().find(|o| o.id == owner_id) {
        if let Some(car) = owner.cars.iter_mut().find(|c| c.id == car_id) {
            car.name = updated.name;
            car.model = updated.model;
            car.year = updated.year;
            car.registration_number = updated.registration_number;

            save_owners(&owners).await;
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Delete car
pub async fn delete_car(
    Path((owner_id, car_id)): Path<(String, String)>,
    State(state): State<SharedState>,
) -> StatusCode {
    let thread_id = thread::current().id();
    println!("delete_car request on thread: {:?}", thread_id);
    let mut owners = state.write().await;

    if let Some(owner) = owners.iter_mut().find(|o| o.id == owner_id) {
        let len_before = owner.cars.len();
        owner.cars.retain(|c| c.id != car_id);

        if owner.cars.len() != len_before {
            save_owners(&owners).await;
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    } else {
        StatusCode::NOT_FOUND
    }
}

// Get all cars
pub async fn get_cars(State(state): State<SharedState>) -> Json<Vec<Car>> {
    let thread_id = thread::current().id();
    println!("get_cars request on thread: {:?}", thread_id);
    let owners = state.read().await;
    let cars: Vec<Car> = owners.iter().flat_map(|o| o.cars.clone()).collect();
    Json(cars)
}

/// Get single car
pub async fn get_car_by_id(
    Path(car_id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Car>, StatusCode> {
    let thread_id = thread::current().id();
    println!("get_car_by_id request on thread: {:?}", thread_id);
    let owners = state.read().await;

    for owner in owners.iter() {
        if let Some(car) = owner.cars.iter().find(|c| c.id == car_id) {
            return Ok(Json(car.clone()));
        }
    }

    Err(StatusCode::NOT_FOUND)
}
