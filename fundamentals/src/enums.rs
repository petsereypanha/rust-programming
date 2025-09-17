#[derive(Debug)]
enum Location {
    India (String),
    US (String),
    UK (String)
}

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
    email_id: String,
    experience: u8,
    location: Location
}

fn is_odd(num:i8) -> Option<bool> {
    if num % 2 == 1 {
        return Some(true);
    } else {
        return None;
    }
}

#[allow(dead_code)]
enum Grade {
    A,
    B,
    C,
    D
}

pub fn enums() {
    let _bangalore = Location::India(String::from("Bangalore"));
    let _delhi = Location::India(String::from("Delhi"));
    let _sanjose = Location::US(String::from("San Jose"));
    let _ny = Location::US(String::from("New York"));
    let _london = Location::UK(String::from("London"));

    let emp1 = Employee {
        name: String::from("Usha"),
        age: 55,
        email_id: String::from("usha@company.com"),
        experience: 25,
        location: _bangalore
    };
    println!("emp1: {:?}", emp1);
    println!("{:?}", is_odd(10));
    println!("{:?}", is_odd(11));
    let grade = Grade::B;
    match grade {
        Grade::A => println!("Excellent"),
        Grade::B => println!("Very good"),
        Grade::C => println!("Good"),
        Grade::D => println!("Poor")
    }
}