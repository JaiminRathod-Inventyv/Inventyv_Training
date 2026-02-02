use std::sync::Mutex;

enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),
}

static GET_COUNT: Mutex<u32> = Mutex::new(0);
static POST_COUNT: Mutex<u32> = Mutex::new(0);
static DELETE_COUNT: Mutex<u32> = Mutex::new(0);
static TOTAL_COUNT: Mutex<u32> = Mutex::new(0);

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
    }

    let total_count = TOTAL_COUNT.lock().unwrap();

    println!("Total requests: {}", *total_count);
}

fn handle_request(req: &Request) -> String {
    {
        let mut total = TOTAL_COUNT.lock().unwrap();
        *total += 1;
    }

    match req {
        Request::Get { endpoint } => {
            let mut count = GET_COUNT.lock().unwrap();
            *count += 1;
            format!(" [GET] : {}", endpoint)
        }
        Request::Post {
            endpoint,
            payload_size,
        } => {
            let mut count = POST_COUNT.lock().unwrap();
            *count += 1;
            format!(" [POST] : {}  [payload size] : {}", endpoint, payload_size)
        }
        Request::Delete(id) => {
            let mut count = DELETE_COUNT.lock().unwrap();
            *count += 1;
            format!(" [DELETE] : {}", id)
        }
    }
}
