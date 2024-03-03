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
#[derive(Debug)]
enum PersinId {
    Passport,
    IdentityCard,
}

struct Person {
    name: String,
    last_name: String,
    age: u32,
    id: PersinId,
}

impl Person {
    fn new() -> Person {
        Person {
            //returns auto
            name: "Default".to_string(),
            last_name: "Test".to_string(),
            age: 21,
            id: PersinId::Passport,
        }
    }

    fn from(name: String, last_name: String, new_age: u32, id: PersinId) -> Person {
        Person {
            name,
            last_name,
            age: new_age,
            id,
        }
    }

    fn some_function() {
        println!("func");
    }

    fn display_age(self: &Self) {
        //&self
        println!("Current Age: {}", self.age)
    }

    fn change_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}

fn main() {
    let mut person = Person {
        name: "Filip".to_string(),
        last_name: "Suha".to_string(),
        age: 30,
        id: PersinId::Passport,
    };

    let mut person2 = Person::new();
    let person3 = Person::from(
        String::from("mert"),
        String::from("suha"),
        21,
        PersinId::Passport,
    );

    person.change_age(22);

    let num_3 = Box::new(100);

    println!("{}", num_3);
}

fn check_person_id(id: PersinId){

    match id {
        
    }
}