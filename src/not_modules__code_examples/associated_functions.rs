struct User {
    name: String,
    age: u8,
    activate: bool,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        Self { 
            name, 
            age,
            activate: true
        }
    }

    fn deactivate(&mut self) {
        self.activate = false;
    }
}

fn main() {
    let mut user = User::new("Jeff".to_string(), 33);
    user.deactivate();
    println!("User name: {}", user.name);
    println!("User age: {}", user.age);
    println!("User activation status: {}", user.activate);
}