use std::{io};
mod file_handler;

fn main() {
    println!("Welcher Tag soll ausgeführt werden?");

    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_n) => {
            let len = input.len();
            let mut input_value = input.to_string();
            
            //remove linebreak
            input_value.truncate(len - 2);
            match input_value.as_str() {
                "1" => print!("Du hast die 1 gewaehlt"),
                _ => println!("Trage eine Zahl von 1 bis 24 und nicht {0}!", input)
            }
        }
        Err(_error) => println!("{0}", _error)
    };
}