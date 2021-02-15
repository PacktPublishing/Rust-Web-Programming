// this function demonstrates a function where both inputs have the same lifetimes
fn get_highest<'a>(first_number: &'a i8, second_number: &'a i8) -> &'a i8 {
    if first_number > second_number {
        first_number
    } else {
        second_number
    }
}

// this function demonstrates a function where both inputs have different lifetimes
fn filter<'a, 'b>(first_number: &'a i8, second_number: &'b i8) -> &'a i8 {
    if first_number < second_number {
        &0
    } else {
        first_number
    }
}


fn main() {
    let one: i8 = 1;
    let long_lasting_outcome: &i8;
    {
        let two: i8 = 2;

        // outcome cannot be relied on outside of this inner scope
        let outcome: &i8 = get_highest(&one, &two);

        // long_lasting_outcome can be use in the outer scope
        long_lasting_outcome = filter(&one, &two);
        println!("{}", outcome);
    }
    println!("{}", long_lasting_outcome);
}
