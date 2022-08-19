use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::Read;
fn read_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("{}", s);
    Ok(s)
}
fn main() {
    let mut file = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Problem creating the file: {:?}", err);
            })
        } else {
            panic!("Problem opening the file: {:?}", err);
        }
    });
    read_from_file();
}
