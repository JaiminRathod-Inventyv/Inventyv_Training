use std::sync::RwLock;

enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),
}

static GET_COUNT: RwLock<u32> = RwLock::new(0);
static POST_COUNT: RwLock<u32> = RwLock::new(0);
static DELETE_COUNT: RwLock<u32> = RwLock::new(0);
static TOTAL_COUNT: RwLock<u32> = RwLock::new(0);

pub fn process_requests() {
    let requests = [
        Request::Get {
            endpoint: "/api/car".to_string(),
        },
        Request::Post {
            endpoint: "/api/car".to_string(),
            payload_size: 100,
        },
        Request::Delete(2),
        Request::Get {
            endpoint: "/api/car".to_string(),
        },
        Request::Delete(2),
        Request::Get {
            endpoint: "/api/car".to_string(),
        },
    ];
    for req in requests {
        let response = handle_request(&req);
        // println!("{}", response);
    }
    let total_count = TOTAL_COUNT.read().unwrap();
    println!("total requests: {}", *total_count);
}

fn handle_request(req: &Request) -> String {
    let mut total = TOTAL_COUNT.write().unwrap();
    *total += 1;

    match req {
        Request::Get { endpoint } => {
            let mut count = GET_COUNT.write().unwrap();
            *count += 1;
            format!(" [GET] : {}", endpoint)
        }
        Request::Post {
            endpoint,
            payload_size,
        } => {
            let mut count = POST_COUNT.write().unwrap();
            *count += 1;
            format!(" [POST] : {}  [payload size] : {}", endpoint, payload_size)
        }
        Request::Delete(id) => {
            let mut count = DELETE_COUNT.write().unwrap();
            *count += 1;
            format!(" [DELETE] : {}", id)
        }
    }
}
