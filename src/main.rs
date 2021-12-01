use std::{io};

fn main() {
    println!("Welcher Tag soll ausgeführt werden?");

    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_n) => {
            match input.as_str() {
                "1" => println!("You selected the first day!"),
                _ => println!("Trage eine Zahl von 1 bis 24")
            }
        }
        Err(_error) => println!("{0}", _error)
    };
}
