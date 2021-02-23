mod to_do;

use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

fn main() {
    let done: Done = Done::new(String::from("shopping"));
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);

    let pending: Pending = Pending::new(String::from("laundry"));
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);
}
