// this function returns the two outcomes from Result => Ok and Err
fn error_check(check: bool) -> Result<i8, &'static str> {
    if check == true {
        return Err("this is an error")
    } else {
        return Ok(1)
    }
}

// this function takes a result and handles it with a match statement
fn describe_result(result: Result<i8, &'static str>) {
    match result {
        Ok(x) => println!("it's a result of: {}", x),
        Err(x) => println!("{}", x)
    }
}


fn main() {
    // unsafe running of function with the direct unwrap
    let result: i8 = error_check(false).unwrap();
    println!("{}", result);

    // does not directly handle result but instead passes it to describe_result
    let safe_result: Result<i8, &'static str> = error_check(true);
    describe_result(safe_result);

    // unsafe running with direct unwrap and extra signposting via the expect function
    let _result: i8 = error_check(true).expect("this has been caught");

    let safe_result: Result<i8, &'static str> = error_check(true);

    // checks to see if error and prints if so
    if safe_result.is_err() {
        println!("it's another error");
    }
}
