use rand::prelude::*;
use std::env;

/// This function generates a float number using a number generator passed into the function.
///
/// # Arguments
/// * generator (&mut ThreadRng): the random number generator to generate the random number
///
/// # Returns
/// (f64): random number between 0 -> 10
fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.gen();
    return placeholder * 10.0
}

/// This trait defines the struct to be a user.
trait IsUser {

    /// This function proclaims that the struct is a user.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (bool) true if user, false if not
    fn is_user() -> bool {
        return true
    }
}

/// This struct defines a user
///
/// # Attributes
/// * name (String): the name of the user
/// * age (i8): the age of the user
struct User {
    name: String,
    age: i8
}

impl IsUser for User {}

fn main() {
    // builds a random generator
    let mut rng: ThreadRng = rand::thread_rng();

    // gets a random number from the generate_float function
    let random_number = generate_float(&mut rng);
    println!("{}", random_number);

    // collect parameters from the environment
    let args: Vec<String> = env::args().collect();

    // get the path to the binary being run
    let path: &str = &args[0];

    // check to see if binary being run is either debug or release
    if path.contains("/debug/") {
        println!("The development app is running");
    }
    else if path.contains("/release/") {
        println!("The production server is running");
    }
    // we throw an error if the binary is not debug or release
    else {
        panic!("The setting is neither debug or release");
    }

    // prints the vector containing the args passed into the program
    println!("{:?}", args);
}
