// here we use the stdn libary to access file system
use std::fs::{self, File};
use std::io;
use std::io::ErrorKind;
use std::io::Read;
// Error handling in Rust
fn main() {
    // panic! macro is used to panic the program
    //example : panic!("crash and burn like spongebob in that one episode");
    // Result enum
    enum Result<T, E> {
        Ok(T),  // success
        Err(E), // error
    }
    let f = File::open("hello.txt");
    // match is used to match the result of the function
    let f = match f {
        // if successful, return the file
        Ok(file) => file,
        // if error, return the error
        // here we use the std::io::ErrorKind enum to match the error
        Err(error) => match error.kind() {
            // if theres an error returning the file then we create one here
            ErrorKind::NotFound => match File::create("hello.text") {
                // we return it if successful
                Ok(fl) => fl,
                // if error, return the error with panic!
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // and if ErrorKind doesnt work this is our backup!
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    // (Heres a more simpler way to do the same thing)

    let f1 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    read_from_file();
    read_from_file_one_liner();
}
// this function is used to read from a file
// returning a string or an error
fn read_from_file() -> Result<String, io::Error> {
    // we prep our string
    let mut s = String::new();
    /*
     * we use the File::open function to open the specifid file
     * we add ? to see if the file is opened or not
     * if not we throw an error
     * then we try to read from our file
     * if tha doesnt work we throw an error there aswell
     */
    File::open("hello.text")?.read_to_string(&mut s)?;
    // finally then we return the variable s with its value
    Ok(s)
}
/*
 here we simplified the above code block by using fs directly up top
 example: use std::fs::{self,File};
*/
fn read_from_file_one_liner() -> Result<String, io::Error> {
    // this has a built in Result enum
    fs::read_to_string("hello.text")
}
