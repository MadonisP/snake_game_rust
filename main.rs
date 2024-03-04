/* use RustSnake::Person;
use RustSnake::Animal; */

use RustSnake::*;
use RustSnake::{Animal, Person};



fn main() {
/*     let mut person = Person  {
        name: "Filip".to_string(),
        last_name: "Suha".to_string(),
        age: 30,
        id: PersinId::Passport(21),
    }; */

    let mut person2 = Person::new();
 /*    let person3 = Person::from(
        String::from("mert"),
        String::from("suha"),
        21,
        PersinId::Passport(21),
    ); */

    person2.change_age(22);

    let num_3 = Box::new(100);

    println!("{}", num_3);
    let animal = Animal("dog".to_string(), 10, "golden".to_string());
    //Person::alert_something();
    /* person2.alert_something(); */
    /* check_person_id(person2.id); */
    log_info(person2);
}


