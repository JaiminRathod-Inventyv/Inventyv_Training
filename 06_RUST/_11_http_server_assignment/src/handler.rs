use crate::model::Owner;

pub fn load_owners() -> Vec<Owner> {
    let data = std::fs::read_to_string("owners.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

pub async fn save_owners(owners: &Vec<Owner>) {
    let data = serde_json::to_string_pretty(owners).unwrap();
    std::fs::write("owners.json", data).expect("Unable to write to file");
}