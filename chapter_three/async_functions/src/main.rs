use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
use std::{thread, time};
use std::vec::Vec;
use async_std;


async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2
}


fn main() {

    let now = time::Instant::now();

    // defines a future
    let future_one = do_something(1);

    // holds the program for the result of the first future
    let outcome = block_on(future_one);
    println!("time elapsed {:?}", now.elapsed());
    println!("here is the result: {}", outcome);

    // defines the async block for multiple futures (just like an async function)
    let second_outcome = async {
        // defines two futures
        let future_two = do_something(2);
        let future_three = do_something(3);
        // waits for both futures to complete in sequence
        return join!(future_two, future_three)
    };

    let now = time::Instant::now();
    // holds the program for the result from the async block
    let result = block_on(second_outcome);
    println!("time elapsed {:?}", now.elapsed());
    println!("here is the result: {:?}", result);


    let third_outcome = async {
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);
        futures_vec.push(future_four);
        futures_vec.push(future_five);

        // applies the spawn async tasks for all futures and collect them into a vector
        let handles = futures_vec.into_iter().map(async_std::task::spawn).collect::<Vec<_>>();
        let results = join_all(handles).await;
        return results
    };

    let now = time::Instant::now();
    let result = block_on(third_outcome);
    println!("time elapsed for join vec {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);
}
