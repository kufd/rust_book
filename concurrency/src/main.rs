use std::collections::hash_map::Entry;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    handle_deadlock_not_blocked_lock_and_retry();
}

fn deadlock() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);
    let handle1 = thread::spawn(move || {
        let mut num1 = counter1_clone.lock().unwrap();
        println!("thread1::counter1 locked");
        thread::sleep(Duration::from_secs(1));
        println!("thread1::waiting for counter2 lock");
        let mut num2 = counter2_clone.lock().unwrap();
        println!("thread1::counter2 locked");

        *num1 += 1;
        *num2 += 1;
    });

    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);
    let handle2 =thread::spawn(move || {
        let mut num2 = counter2_clone.lock().unwrap();
        println!("thread2::counter2 locked");
        thread::sleep(Duration::from_secs(1));
        println!("thread2::waiting for counter1 lock");
        let mut num1 = counter1_clone.lock().unwrap();
        println!("thread2::counter1 locked");


        *num1 += 1;
        *num2 += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Result1: {}", *counter1.lock().unwrap());
    println!("Result2: {}", *counter2.lock().unwrap());
}

fn handle_deadlock_sorting_resources() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);
    let handle1 = thread::spawn(move || {
        let mut num1 = counter1_clone.lock().unwrap();
        println!("thread1::counter1 locked");
        thread::sleep(Duration::from_secs(1));
        println!("thread1::waiting for counter2 lock");
        let mut num2 = counter2_clone.lock().unwrap();
        println!("thread1::counter2 locked");

        *num1 += 1;
        *num2 += 1;
    });

    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);
    let handle2 = thread::spawn(move || {
        let mut num1 = counter1_clone.lock().unwrap();
        println!("thread2::counter1 locked");
        thread::sleep(Duration::from_secs(1));
        println!("thread2::waiting for counter2 lock");
        let mut num2 = counter2_clone.lock().unwrap();
        println!("thread2::counter2 locked");


        *num1 += 1;
        *num2 += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Result1: {}", *counter1.lock().unwrap());
    println!("Result2: {}", *counter2.lock().unwrap());
}

fn handle_deadlock_not_blocked_lock_and_retry() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);
    let handle1 = thread::spawn(move || {
        for attempt_number in 0..10 {
            let num1_loc_result = counter1_clone.try_lock();
            let mut num1 = match num1_loc_result {
                Ok(num) => num,
                Err(error) => {
                    println!("thread1::counter1 cannot lock(attempt number {}): {}", attempt_number, error);
                    continue;
                },
            };
            println!("thread1::counter1 locked");

            thread::sleep(Duration::from_secs(1));

            println!("thread1::waiting for counter2 lock");
            let num2_loc_result = counter2_clone.try_lock();
            let mut num2 = match num2_loc_result {
                Ok(num) => num,
                Err(error) => {
                    println!("thread1::counter2 cannot lock(attempt number {}): {}", attempt_number, error);
                    continue;
                },
            };
            println!("thread1::counter2 locked");

            *num1 += 1;
            *num2 += 1;
            return;
        }

        println!("thread1::all retries are failed");
    });

    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);
    let handle2 =thread::spawn(move || {
        for attempt_number in 0..10 {
            println!("thread2::waiting for counter2 lock");
            let num2_loc_result = counter2_clone.try_lock();
            let mut num2 = match num2_loc_result {
                Ok(num) => num,
                Err(error) => {
                    println!("thread2::counter2 cannot lock(attempt number {}) {}", attempt_number, error);
                    continue;
                },
            };
            println!("thread2::counter2 locked");

            thread::sleep(Duration::from_secs(1));

            let num1_loc_result = counter1_clone.try_lock();
            let mut num1 = match num1_loc_result {
                Ok(num) => num,
                Err(error) => {
                    println!("thread2::counter1 cannot lock(attempt number {}) {}", attempt_number, error);
                    continue;
                },
            };
            println!("thread2::counter1 locked");

            *num1 += 1;
            *num2 += 1;
            return;
        }

        println!("thread2::all retries are failed");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Result1: {}", *counter1.lock().unwrap());
    println!("Result2: {}", *counter2.lock().unwrap());
}