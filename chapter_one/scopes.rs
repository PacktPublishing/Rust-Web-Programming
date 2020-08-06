// this function alters the mut borrow by de-referencing via the * operator
fn alter_number(number: &mut i8) {
    *number += 1
}

fn print_number(number: i8) {
    println!("{}", number);
}

fn main() {
    let mut one: i8 = 1;
    print_number(one);
    alter_number(&mut one);
    println!("{}", one);

    let two: String = String::from("two");

    // these brackets start a new scope
    {
        println!("{}", &two);
        let three: String = String::from("three");
    } // =====> lifetime ends for three here so three is no longer available
    // two is still available
    println!("{}", two);
}
