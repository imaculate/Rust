fn main() {
    let s = String::from("The Butler");
    let mut s2 = s;
    // s has been moved, its no longer usable
    println!("s2 is {}", s2);

    println!("Calculated length is {}", calculate_length(&s2));

    change(&mut s2);

    println!("Changed value is now {}", s2);

    // borrowing with mutable references only per scope
    {
        let _r1 = &mut s2;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let rr2 = &mut s2;
    println!("Mutable reference {}", rr2);

    // borrowing with immutable references, can borrow mutable ref after immutable references
    let r1 = &s2; // no problem
    let r2 = &s2; // no problem
    // let r3 = &mut s2; // BIG PROBLEM

    println!("Immutable references {} and {}", r1, r2);

    let r3 = &mut s2;
    println!("Mutable reference {}", r3);
}

fn calculate_length(s : &String) -> usize
{
    s.len()
}

fn change(some_string : &mut String)
{
    some_string.push_str(" has arrived!");
}

//this method leads to compile error due to dangling references to s
// it does out of scope and dropped at end of function but garbage ref will
// be returned
//fn dangle_ref() -> &String
//{
//   let s = String::from("hello!");
//   &s
//}