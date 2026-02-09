use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

trait DisplayItem {
    fn display(&self) -> String;
}
struct Inventory<'a, T>
where
    T: DisplayItem + Clone + 'a,
{
    items: HashMap<String, &'a T>,
}

impl DisplayItem for String {
    fn display(&self) -> String {
        format!("{}", self)
    }
}

impl<'a, T> Inventory<'a, T>
where
    T: DisplayItem + Clone + 'a,
{
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }

    fn add_items(&mut self, id: String, item: &'a T) -> Result<(), InventoryError> {
        //check for InvalidId
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }

        // check for id exist or not
        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateItem(id));
        }

        self.items.insert(id, item);
        Ok(())
    }

    fn find_item_by_item_id(&self, id: String) -> Result<&'a T, InventoryError> {
        match self.items.get(&id) {
            Some(item) => Ok(item),
            None => Err(InventoryError::ItemNotFound(id)),
        }
    }

   
}

#[derive(Debug)]
enum InventoryError {
    ItemNotFound(String),
    DuplicateItem(String),
    InvalidId,
}

impl Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::ItemNotFound(id) => write!(f, "item is not found with ID : {id}"),
            InventoryError::DuplicateItem(id) => write!(f, "item already exists with ID : {id}"),
            InventoryError::InvalidId => write!(f, "provided item-id is invalid"),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Product {
    product_id: String,
    product_name: String,
}

impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("{:?}", self)
    }
}

fn main() {
    println!("-------- Inventory T as String-------");
    let mut inventory1 = Inventory::<String>::new();

    let item1 = "item1".to_string();
    match inventory1.add_items("I01".to_string(), &item1) {
        Ok(()) => println!("item added successfully!"),
        Err(e) => println!("{}", e),
    }
    let item2 = "item2".to_string();
    match inventory1.add_items("I02".to_string(), &item2) {
        Ok(()) => println!("item added successfully!"),
        Err(e) => println!("{}", e),
    }
    let item3 = "item3".to_string();
    match inventory1.add_items("I03".to_string(), &item3) {
        Ok(()) => println!("item added successfully!"),
        Err(e) => println!("{}", e),
    }
    let item4 = "item4".to_string();
    match inventory1.add_items("".to_string(), &item4) {
        Ok(()) => println!("item added successfully!"),
        Err(e) => println!("{}", e),
    }

    match inventory1.find_item_by_item_id("I01".to_string()) {
        Ok(item) => print!("item found : \n{}\n", item),
        Err(e) => println!("{}", e),
    }

    let display_inventory = |inv: &Inventory<String>| {
        println!("-------- Displaying Inventory -------");
        if inv.items.is_empty() {
            println!("inventory is empty!!");
            return;
        }
        for (id, item) in &inv.items {
            println!("{} : {}", id, item.display());
        }
    };

    display_inventory(&inventory1);

    println!("-------- Inventory T as Product Struct-------");

    let mut product_inventory = Inventory::<Product>::new();
    let p1 = Product {
        product_id: "p01".to_string(),
        product_name: "Laptop".to_string(),
    };

    match product_inventory.add_items(p1.product_id.clone(), &p1) {
        Ok(()) => println!("product added!"),
        Err(e) => println!("Error: {}", e),
    }
    let p2 = Product {
        product_id: "p02".to_string(),
        product_name: "Phone".to_string(),
    };
    match product_inventory.add_items("p02".to_string(), &p2) {
        Ok(()) => println!("Product added!"),
        Err(e) => println!("Error: {}", e),
    }

    let p3 = Product {
        product_id: "p02".to_string(),
        product_name: "Phone".to_string(),
    };
    match product_inventory.add_items("p02".to_string(), &p3) {
        Ok(()) => println!("Product added!"),
        Err(e) => println!("Error: {}", e),
    }

    let p4 = Product {
        product_id: "p02".to_string(),
        product_name: "Phone".to_string(),
    };
    match product_inventory.add_items("".to_string(), &p4) {
        Ok(()) => println!("Product added!"),
        Err(e) => println!("Error: {}", e),
    }

    match product_inventory.find_item_by_item_id("p01".to_string()) {
        Ok(item) => print!("item found : \n{:?}\n", item),
        Err(e) => println!("{}", e),
    }

    let display_product_inventory = |inv: &Inventory<Product>| {
        println!("-------- Displaying Inventory -------");
        if inv.items.is_empty() {
            println!("inventory is empty!!");
            return;
        }
        for (id, product) in &inv.items {
            println!("{} : {:?}", id, product);
        }
    };

    display_product_inventory(&product_inventory);
}
