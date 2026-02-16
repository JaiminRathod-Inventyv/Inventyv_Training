use crate::{SharedState, api::*};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn app_routes() -> Router<SharedState> {
    Router::new()
        .route("/owners", get(get_owners).post(create_owner))
        .route(
            "/owners/{id}",
            get(get_owner_by_id).put(update_owner).delete(delete_owner),
        )
        .route("/owners/{owner_id}/cars", post(add_car))
        .route(
            "/owners/{owner_id}/cars/{car_id}",
            put(update_car).delete(delete_car),
        )
        .route("/cars", get(get_cars))
        .route("/cars/{id}", get(get_car_by_id))
}
