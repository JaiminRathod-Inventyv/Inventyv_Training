use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, RwLock,
    },
    thread,
    time::SystemTime,
};

use chrono::{DateTime, Utc};
#[derive(Debug)]
#[allow(dead_code)]
struct MultiThread {
    id: i32,
    recordAddedTime: String,
    threadId: String,
}

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let vec: Vec<MultiThread> = Vec::new();
    let mut handlers = Vec::new();
    let data = Arc::new(RwLock::new(vec));

    let data_clone = Arc::clone(&data);
    let handle1 = thread::spawn(move || loop {
        let multi_thread: MultiThread = MultiThread {
            id: COUNTER.fetch_add(1, Ordering::SeqCst),
            recordAddedTime: chrono::DateTime::<chrono::Utc>::from(SystemTime::now()).to_string(),
            threadId: format!("{:?}", thread::current().id()),
        };
        {
            let mut data1 = data_clone.write().unwrap();
            println!("Thread 1: Added record : {:#?}", multi_thread);
            data1.push(multi_thread);
        }
        thread::sleep(std::time::Duration::from_secs(10));
    });
    handlers.push(handle1);

    let data_clone2 = Arc::clone(&data);
    let handle2 = thread::spawn(move || loop {
        {
            let data2 = data_clone2.read().unwrap();
            println!("Thread 2: Current records in memory : {:#?}", *data2);
        }
        thread::sleep(std::time::Duration::from_millis(2000));
    });
    handlers.push(handle2);

    let data_clone3 = Arc::clone(&data);
    let handle3 = thread::spawn(move || loop {
        {
            let mut data3 = data_clone3.write().unwrap();
            data3.retain(|record| {
                let record_time: DateTime<Utc> =
                    DateTime::from_str(&record.recordAddedTime).unwrap();
                let current_time = Utc::now();
                let duration = current_time
                    .signed_duration_since(record_time)
                    .num_seconds();
                if duration > 20 && record.id % 2 == 0 {
                    println!("Thread 3: Removing record : {:#?}", record);
                    false
                } else {
                    true
                }
            })
        }
        thread::sleep(std::time::Duration::from_millis(1000));
    });
    handlers.push(handle3);

    let data_clone4 = Arc::clone(&data);
    let handle4 = thread::spawn(move || loop {
        {
            let mut data4 = data_clone4.write().unwrap();
            data4.retain(|record| {
                let record_time: DateTime<Utc> =
                    DateTime::from_str(&record.recordAddedTime).unwrap();
                let current_time = Utc::now();
                let duration = current_time
                    .signed_duration_since(record_time)
                    .num_seconds();
                if duration > 20 && record.id % 2 != 0 {
                    println!("Thread 3: Removing record : {:#?}", record);
                    false
                } else {
                    true
                }
            })
        }
        thread::sleep(std::time::Duration::from_millis(5000));
    });
    handlers.push(handle4);

    let data_clone5 = Arc::clone(&data);
    let handle5 = thread::spawn(move || loop {
        {
            let data5 = data_clone5.read().unwrap();
            let even_count = data5.iter().filter(|record| record.id % 2 == 0).count();
            println!("Thread 5: Count of even id records : {}", even_count);
        }
        thread::sleep(std::time::Duration::from_millis(5000));
    });
    handlers.push(handle5);

    let data_clone6 = Arc::clone(&data);
    let handle6 = thread::spawn(move || loop {
        {
            let data6 = data_clone6.read().unwrap();
            let even_count = data6.iter().filter(|record| record.id % 2 != 0).count();
            println!("Thread 6: Count of odd id records : {}", even_count);
        }
        thread::sleep(std::time::Duration::from_millis(5000));
    });
    handlers.push(handle6);

    for handle in handlers {
        handle.join().unwrap();
    }
}
