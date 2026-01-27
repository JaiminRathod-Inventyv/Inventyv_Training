use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
struct Car {
    brand: String,
    model: String,
    year: u16,
    color: String,
    engine: Engine,
    owner: Owner,
}

#[derive(Debug, Serialize)]
struct Engine {
    fuel_type: String,
    mileage: u32,
}

#[derive(Debug, Serialize)]
struct Owner {
    name: String,
    age: u8,
}

impl Car {
    fn get_brand(&self) -> String {
        self.brand.clone()
    }

    fn get_model(&self) -> String {
        self.model.clone()
    }

    fn get_year(&self) -> u16 {
        self.year
    }

    fn get_color(&self) -> String {
        self.color.clone()
    }

    fn get_mileage(&self) -> u32 {
        self.engine.mileage
    }

    fn get_fuel_type(&self) -> String {
        self.engine.fuel_type.clone()
    }

    fn get_owner_name(&self) -> String {
        self.owner.name.clone()
    }

    fn get_owner_age(&self) -> u8 {
        self.owner.age
    }

    fn get_engine_info(&self) -> String {
        format!(
            "Fuel Type: {}, Mileage: {}",
            self.engine.fuel_type, self.engine.mileage
        )
    }

    fn get_owner_info(&self) -> String {
        format!("Name: {}, Age: {}", self.owner.name, self.owner.age)
    }

    fn set_brand(&mut self, brand: &str) {
        self.brand = brand.to_string();
    }

    fn set_model(&mut self, model: &str) {
        self.model = model.to_string();
    }

    fn set_year(&mut self, year: u16) {
        self.year = year;
    }

    fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }

    fn set_mileage(&mut self, mileage: u32) {
        self.engine.mileage = mileage;
    }

    fn set_fuel_type(&mut self, fuel_type: &str) {
        self.engine.fuel_type = fuel_type.to_string();
    }

    fn set_owner_name(&mut self, name: &str) {
        self.owner.name = name.to_string();
    }

    fn set_owner_age(&mut self, age: u8) {
        self.owner.age = age;
    }

    fn set_engine_info(&mut self, fuel_type: &str, mileage: u32) {
        self.engine.fuel_type = fuel_type.to_string();
        self.engine.mileage = mileage;
    }

    fn set_owner_info(&mut self, name: &str, age: u8) {
        self.owner.name = name.to_string();
        self.owner.age = age;
    }

    fn get_car_info(&self) -> String {
        format!(
            "Brand: {}, Model: {}, Year: {}, Color: {}, Mileage: {}, Fuel Type: {} , Owner Name: {}, Owner Age: {}",
            self.brand,
            self.model,
            self.year,
            self.color,
            self.engine.mileage,
            self.engine.fuel_type,
            self.owner.name,
            self.owner.age
        )
    }

    fn get_car_info_with_args(
        brand: &str,
        model: &str,
        year: u16,
        color: &str,
        mileage: u32,
        fuel_type: &str,
        owner_name: &str,
        owner_age: u8,
    ) -> String {
        format!(
            "Brand: {}, Model: {}, Year: {}, Color: {}, Mileage: {}, Fuel Type: {}, Owner Name: {}, Owner Age: {}",
            brand, model, year, color, mileage, fuel_type, owner_name, owner_age
        )
    }

    fn set_car_info_with_self(
        &mut self,
        brand: &str,
        model: &str,
        year: u16,
        color: &str,
        mileage: u32,
        fuel_type: &str,
        owner_name: &str,
        owner_age: u8,
    ) {
        self.brand = brand.to_string();
        self.model = model.to_string();
        self.year = year;
        self.color = color.to_string();
        self.engine.mileage = mileage;
        self.engine.fuel_type = fuel_type.to_string();
        self.owner.name = owner_name.to_string();
        self.owner.age = owner_age;
    }

    fn set_car_info_wo_self(
        brand: &str,
        model: &str,
        year: u16,
        color: &str,
        mileage: u32,
        fuel_type: &str,
        owner_name: &str,
        owner_age: u8,
    ) -> Car {
        Car {
            brand: brand.to_string(),
            model: model.to_string(),
            year,
            color: color.to_string(),
            engine: Engine {
                mileage,
                fuel_type: fuel_type.to_string(),
            },
            owner: Owner {
                name: owner_name.to_string(),
                age: owner_age,
            },
        }
    }
}

fn main() {
    let mut car = Car {
        brand: String::from("BMW"),
        model: String::from("M3"),
        year: 2023,
        color: String::from("Black"),
        engine: Engine {
            fuel_type: String::from("Diesel"),
            mileage: 1500,
        },
        owner: Owner {
            name: String::from("Jaimin"),
            age: 21,
        },
    };

    let car_json_string = serde_json::to_string(&car).unwrap();

    println!("Serialized car:\n{}", car_json_string);
}
