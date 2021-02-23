use std::{thread, time};
use std::thread::JoinHandle;


/// This function prints a statement and returns an integer to demonstrate a running process for a
///thread.
///
/// # Arguments
/// * number (i8): number to denote what thread is running the function.
///
/// # Returns
/// * (i8): fixed number
fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2
}

fn main() {

    // start the timer
    let now = time::Instant::now();

    // fire a few functions in a sequential manner
    let one: i8 = do_something(1);
    let two: i8 = do_something(2);
    let three: i8 = do_something(3);

    // print the outcomes
    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", one + two + three);

    // start the timer again
    let now = time::Instant::now();

    // spawn a few functions with the same function
    let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
    let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
    let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));

    // get the results from the threads
    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();

    // print the outcomes again from the threads
    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", result_one.unwrap() + result_two.unwrap() + result_three.unwrap());
}
