pub fn run(){
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_age = true;

    if age >= 21 && check_id || knows_person_age {
        println!("What do you want to drink?");
    } else if  age < 21 && check_id {
        println!("Sorry, you have to leave");
    } else {
        println!("I need your ID");
    }

    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);

}