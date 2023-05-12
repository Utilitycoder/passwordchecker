use passwordchecker::check_passwords;
use std::io;

fn main() {
    let mut input = String::from("");
    println!("Enter a password to check:");
    io::stdin().read_line(&mut input).unwrap();
    let password = input;

    let result = check_passwords::check_password(&password);
    match result {
        Ok(found) => {
            if found {
                println!("Password found!");
            } else {
                println!("Password not found!");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
