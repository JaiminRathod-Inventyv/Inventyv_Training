use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner{
    #[serde(default)]
    pub id : String,
    pub name : String,
    pub email : String,
    pub cars : Vec<Car>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car{
    #[serde(default)]
    pub id : String,
    pub name : String,
    pub model : String,
    pub year : i32,
    pub registration_number : String,
}