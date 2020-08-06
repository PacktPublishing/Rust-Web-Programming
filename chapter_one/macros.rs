// defines a generic struct
struct Coordinate <T> {
    x: T,
    y: T
}

// defines a basic macro that capitalizes the first letter of a String
macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        $a = v.into_iter().collect();
    }
}

// defines a struct that can be copied and debugged
#[derive(Debug, Clone, Copy)]
struct CopyCoordinate {
    x: i8,
    y: i8
}

// this function demonstrates that the copy trait on the CopyCoordinate enables us to use the
// CopyCoordinate again after being used here
fn print(point: CopyCoordinate) {
    println!("{} {}", point.x, point.y);
}



fn main() {
    // shows that different data types can be made with this generic struct
    let one = Coordinate{x: 50, y: 50};
    let two = Coordinate{x: 500, y: 500};
    let three = Coordinate{x: 5.6, y: 5.6};

    let mut x = String::from("test");
    // fires our macro
    capitalize!(x);

    // makes the x capitalized without us having to define it using let
    println!("{}", x);


    let test = CopyCoordinate{x: 1, y:2};
    print(test);
    println!("{}", test.x)

}
