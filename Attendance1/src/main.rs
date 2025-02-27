use std::io::{self, Write};

struct Person {
    name: String,
    sid: u32,
}

fn main() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What is your Student ID? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let sid = buffer.trim().parse().unwrap();

    let person = Person { name, sid };
    println!("Hi {}, your Student ID is {}!", person.name, person.sid);
}