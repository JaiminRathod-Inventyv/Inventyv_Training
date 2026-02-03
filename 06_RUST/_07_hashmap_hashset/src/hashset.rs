use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
struct Car {
    car_id: u32,
    brand: String,
}

pub fn run() {
    let mut car_set: HashSet<Car> = HashSet::new();
    // insert method for HashSet
    car_set.insert(Car {
        car_id: 1,
        brand: String::from("BMW"),
    });
    car_set.insert(Car {
        car_id: 2,
        brand: String::from("Toyota"),
    });
    println!("Initial HashSet: {:#?}", car_set);

    // clone() for HashSet
    let car_set_clone = car_set.clone();
    println!("Cloned HashSet: {:#?}", car_set_clone);

    // try_reserve for HashSet
    let reserve_try = car_set.try_reserve(2); // it will reserve space for 2 more elements
    match reserve_try {
        Ok(_) => println!("Successfully reserved space for 2 more elements."),
        Err(e) => println!("Failed to reserve space: {}", e),
    }
    car_set.insert(Car {
        car_id: 3,
        brand: String::from("Honda"),
    });
    car_set.insert(Car {
        car_id: 4,
        brand: String::from("Ford"),
    });
    println!(
        "HashSet after reserving space and inserting more elements: {:#?}",
        car_set
    );

    // take() for HashSet
    let removed_car = car_set.take(&Car {
        car_id: 2,
        brand: String::from("Toyota"),
    });
    match removed_car {
        Some(car) => println!("Taken car: {:#?}", car),
        None => println!("Car not found in the set."),
    }
    println!("HashSet after taking a car: {:#?}", car_set);

    // retain() for HashSet
    car_set.retain(|car| car.car_id % 2 == 0); // retain only cars with even car_id

    println!("HashSet after retaining cars with even car_id: {:#?}", car_set);

    // extend() for HashSet
    let mut new_cars: HashSet<Car> = HashSet::new();
    new_cars.insert(Car {
        car_id: 5,
        brand: String::from("Chevrolet"),
    });
    new_cars.insert(Car {
        car_id: 6,
        brand: String::from("Nissan"),
    });
    car_set.extend(new_cars);
    println!("HashSet after extending with new cars: {:#?}", car_set);
    
}
