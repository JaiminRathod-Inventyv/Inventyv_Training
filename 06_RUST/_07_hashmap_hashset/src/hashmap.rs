use std::collections::{HashMap};

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Car {
    car_id: u32,
    brand: String,
}

pub fn run() {
    let mut car_map: HashMap<u32, Car> = HashMap::new();

    // insert method for HashMap
    car_map.insert(
        1,
        Car {
            car_id: 1,
            brand: String::from("BMW"),
        },
    );
    car_map.insert(
        2,
        Car {
            car_id: 2,
            brand: String::from("Toyota"),
        },
    );

    // println!("{:#?}", car_map);

    // clone() for HashMap
    let mut cloned_map = car_map.clone();
    // let mut copied_map = &mut car_map;
    println!("Cloned HashMap: {:#?}", cloned_map);
    cloned_map.insert(
        3,
        Car {
            car_id: 3,
            brand: String::from("Honda"),
        },
    );
    println!("after insertion - cloned HashMap: {:#?}", cloned_map);
    println!("original HashMap remains unchanged: {:#?}", car_map);

    // try_reserve for HashMap
    let reverse_try = car_map.try_reserve(2); //  it will reserve space for 2 more elements
    match reverse_try {
        // try_reserve returns a Result type so we need to handle both success and failure cases
        Ok(_) => println!("Successfully reserved space for 2 more elements."),
        Err(e) => println!("Failed to reserve space: {}", e),
    }
    car_map.insert(
        3,
        Car {
            car_id: 3,
            brand: String::from("Ford"),
        },
    );
    car_map.insert(
        4,
        Car {
            car_id: 4,
            brand: String::from("Chevrolet"),
        },
    );
    car_map.insert(
        5,
        Car {
            car_id: 5,
            brand: String::from("Nissan"),
        },
    );
    println!(
        "HashMap after reserving space and inserting more elements: {:#?}",
        car_map
    );

    // remove() {similar to take in hashset}
    let taken_car = car_map.remove(&1);
    match &taken_car {
        Some(car) => println!("Successfully taken car with car_id {:#?}", car.car_id),
        None => println!("No car found with the given car_id to take."),
    }

    // retain() for HashMap
    car_map.retain(
        |key , car| car.car_id % 2 == 0
    ); // retain only cars with even car_id keys
    println!("HashMap after retaining cars with even car_id: {:#?}", car_map);

    // extend() for HashMap

    let mut new_cars: HashMap<u32, Car> = HashMap::new();
    new_cars.insert(
        6,
        Car {
            car_id: 6,
            brand: String::from("Audi"),
        },
    );
    new_cars.insert(
        7,
        Car {
            car_id: 7,
            brand: String::from("Mercedes"),
        },
    );
    car_map.extend(new_cars);
    println!("HashMap after extending with new cars: {:#?}", car_map);

}
