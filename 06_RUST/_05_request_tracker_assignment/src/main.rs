mod mutex;
mod rwlock; 


fn main() {
    println!("using mutex:");
    mutex::process_requests();
    println!("using rwlock:");
    rwlock::process_requests();
}
