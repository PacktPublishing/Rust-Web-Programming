use warp::Filter;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let home = warp::path!("home")
        .map(|| "This is a Warp server built in Rust");

    let greet = warp::path!("greet" / String / i32)
        .map(|name: String, age: i32| {
           return format!("I am {} and {} years old", name, age)
        });

    let add = warp::path!("add" / i32 / i32)
        .map(|one: i32, two: i32| {
            let result: i32 = one + two;
            return warp::reply::json(&result)
        });

    let log = warp::log("to_do::api");

    let routes = warp::get().and(
            home
            .or(greet)
            .or(add)
    ).with(log);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}