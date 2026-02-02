pub mod _01_loops;
pub mod _02_structs_and_methods;

// by making mod.rs in subfolder
pub mod _03_serde;

// by using nested modules without making mod.rs in subfolder
pub mod _05_request_tracker_assignment{
    pub mod main;
    pub mod mutex;
    pub mod rwlock;
}

