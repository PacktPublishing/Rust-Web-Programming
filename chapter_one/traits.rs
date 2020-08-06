// Defines the CanEdit trait
trait CanEdit {
    fn edit(&self) -> () {
        println!("user is editing");
    }
}

// Defines the CanCreate trait
trait CanCreate {
    fn create(&self) -> () {
        println!("user is creating");
    }
}

// Defines the CanDelete trait
trait CanDelete {
    fn delete(&self) -> () {
        println!("user is deleting");
    }
}

// Defines the admin user struct
struct AdminUser {
    name: String,
    password: String,
}

// enables the admin user struct to have CanDelete, CanCreate, and CanEdit traits
impl CanDelete for AdminUser {}
impl CanCreate for AdminUser {}
impl CanEdit for AdminUser {}

// this function allows users with the CanDelete trait implemented on it
fn delete<T: CanDelete>(user: T) -> () {
    user.delete();
}

// defines the base user struct to be inherited
struct BaseUser {
    name: String,
    password: String
}

// defines the general user with the inheritance of the BaseUser struct through composition
struct GeneralUser {
    super_struct: BaseUser,
    team: String
}

impl GeneralUser {

    // the constructor of the GeneralUser
    fn new(name: String, password: String, team: String) -> GeneralUser {
        return GeneralUser{super_struct: BaseUser{name, password}, team: team}
    }
}

// implements the CanEdit trait for the GeneralUser struct
impl CanEdit for GeneralUser {}

impl CanCreate for GeneralUser {
    fn create(&self) -> () {
        println!("{} is creating under a {} team", self.super_struct.name, self.team);
    }
}
