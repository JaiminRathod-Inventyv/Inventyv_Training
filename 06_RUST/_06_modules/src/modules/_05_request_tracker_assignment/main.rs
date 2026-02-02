 use crate::modules::_05_request_tracker_assignment::mutex;
 use crate::modules::_05_request_tracker_assignment::rwlock;


pub fn run() {
    println!("using mutex:");
    mutex::process_requests();
    println!("using rwlock:");
    rwlock::process_requests();
}
