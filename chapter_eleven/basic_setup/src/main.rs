#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/bye/<name>/<age>")]
fn bye(name: String, age: u8) -> String {
    format!("Goodbye, {} year old named {}!", age, name)
}


fn main() {
    rocket::ignite().mount("/", routes![hello, bye]).launch();
}
