/* fn main() {
    let message = String::from("Hello");
    print_message(message);

    let mut mutMessage= String::from("Hello");
    let message_2: &mut String= &mut mutMessage;

    message_2.push_str("test");

    println!("{}", message_2);
    println!("{}", mutMessage);
    let mut a= 10;
    let b= &mut a;

    let b = &&200;
    println!("{}", a==**b);
}

fn print_message(a: String) {
    let mut c = a;

    c.push_str(" World");
    println!("{}",c);

}  */

pub trait Log {
    fn display_info(&self);
    fn alert_something(&self) {
        println!("default impolemantation")
    }
}

#[derive(Debug)]
enum PersinId {
    Passport(u32),
    IdentityCard(u32, u32, u32),
}

pub struct Person {
    name: String,
    last_name: String,
    age: u32,
    id: PersinId,
}

pub struct Animal(pub String, pub u32, pub String);

impl Log for Animal {
    fn display_info(&self) {
        println!("{}", self.0);
    }
}

impl Log for Person {
    fn display_info(&self) {
        println!(
            "{:?} {} {} {}",
            self.id, self.name, self.last_name, self.age
        )
    }
}

impl Person {
    pub fn new() -> Person {
        Person {
            //returns auto
            name: "Default".to_string(),
            last_name: "Test".to_string(),
            age: 21,
            id: PersinId::Passport(21),
        }
    }

    pub fn from(name: String, last_name: String, new_age: u32, id: PersinId) -> Person {
        Person {
            name,
            last_name,
            age: new_age,
            id,
        }
    }

    pub fn some_function() {
        println!("func");
    }

    pub fn display_age(self: &Self) {
        //&self
        println!("Current Age: {}", self.age)
    }

    pub fn change_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}

pub fn check_person_id(id: PersinId) {
    let result = match id {
        PersinId::IdentityCard(x, y, z) => x,
        PersinId::Passport(x) => x,
    };

    println!("res {}", result)
}

pub fn log_info(val: impl Log) {
    val.alert_something();
}
