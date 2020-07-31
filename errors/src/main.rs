use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    // panic!("Crash and Burn!");
    // let v = vec![5, 8 , 90, 45, 76];
    //v[99];
    // Enough panicking!

    // using match
    let _f = match File::open("hello.txt") 
    {
        Ok(file) => file,
        Err(error) => match error.kind()
        {
            ErrorKind::NotFound => match File::create("hello.txt")
            {
                Ok(file) => file,
                Err(error) => panic!("Problem while creating file {:?}", error)
            },
            other_error => panic!("Error occured while opening hello.txt {:?}", other_error)
        },
    };

    // using unwrap or else
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound
        {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Could not create hello.txt {:?}", error);
            })
        }
        else
        {
            panic!("Could not open hello.txt {:?}", error);
        }
    });

    // using unwrap or expect
    let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("Failed while opening hello.txt");

    // returning Result
    let path  = String::from("hello.txt");
    let s = read_text_from_file_match_question_mark(&path);
    println!("Read with ? {} from file", s.unwrap());
    let s = read_from_string_builtin(&path);
    println!("Read with builtin {} from file", s.unwrap());
    let s = read_text_from_file_match(&path);
    println!("Read with match {} from file", s.unwrap());
    Ok(())
}


fn read_text_from_file_match(filepath: &String) -> Result<String, io::Error>
{
    let f = File::open(filepath);
    let mut s = String::new();

    let mut f = match f 
    {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    match f.read_to_string(&mut s)
    {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_text_from_file_match_question_mark(filepath: &String) -> Result<String, io::Error>
{
    let mut s = String::new();
    File::open(filepath)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_string_builtin(filepath: &String) -> Result<String, io::Error>
{
    fs::read_to_string(filepath)
}