use std::thread;
use std::time::Duration;

use concurrency::threaded::{self, concurrency_shared_state};

fn main() {
    threaded::bland_threading();

    println!(".....");
    thread::sleep(Duration::from_millis(2));

    threaded::wait_threading();

    println!(".....");
    thread::sleep(Duration::from_millis(2));

    threaded::move_closure();

    println!(".....");
    thread::sleep(Duration::from_millis(2));
    
    threaded::channel();

    println!(".....");
    thread::sleep(Duration::from_millis(2));

    threaded::channel_ownershp_transference();

    println!(".....");
    thread::sleep(Duration::from_millis(2));
    
    threaded::channel_ownershp_transference();

    println!(".....");
    thread::sleep(Duration::from_millis(2));
    
    concurrency_shared_state::basic();

    println!(".....");
    thread::sleep(Duration::from_millis(2));
    
    concurrency_shared_state::shared_across_threads();
}
