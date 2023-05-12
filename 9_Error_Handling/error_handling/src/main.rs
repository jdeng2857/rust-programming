use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::error::Error;
use std::io::{self, Read};

fn file_errors_with_match() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt"){
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "Problem creating the file: {:?}", e
                    ),
                }
            }
            other_error => {
                panic!(
                    "Problem opening the file: {:?}", other_error
                );
            }
        },
    };
}

fn file_errors_with_closures() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// ? shortcut for propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // alternate one-liner
    // fs::read_to_string("hello.txt");
}

fn open_file_with_result() -> Result<(), Box<dyn Error>>{
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

// Example for guessing game, check input is between 1-100
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Guess value must be between 1 and 100, got {}.", value
            );
        }
        Guess { value }
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {

    // panic example
    // let v = vec![1,2,3];
    // v[99];

    file_errors_with_match();
    file_errors_with_closures();

    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    read_username_from_file();
    open_file_with_result();
    
}
