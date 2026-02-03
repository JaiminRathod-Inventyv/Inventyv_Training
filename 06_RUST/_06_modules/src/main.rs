mod modules;

fn main() {

    println!("--- _01_loops ---");
    modules::_01_loops::run();
    println!("--- _02_structs_and_methods ---");
    modules::_02_structs_and_methods::run();
    println!("--- 03_serde ---");
    modules::_03_serde::_01_serde_serialization::run();
    modules::_03_serde::_02_serde_deserialization::run();
    println!("--- 04_ownership_and_borrowing ---");
    modules::_04_ownership_and_borrowing::run();
    println!("--- 05_request_tracker_assignment ---");
    modules::_05_request_tracker_assignment::main::run();


}
