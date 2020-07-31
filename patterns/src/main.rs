
fn main() {
    let mut stack = vec![1, 2, 3];

    while let Some(s) = stack.pop()
    {
        println!("Retrieved {} from stack", s);
    }

    // irrefutable pattern mismatched
    /*if let x = 5
    {
        println!("x is {}", x);
    }*/

    let x = Some(5);
    let y = 10;

    match x
    {
        Some(50) => println!("x is Some(50)"),
        Some(y) => println!("Got y = {}", y),
        _ => println!("Everything in between")
    }

    println!("In the end x = {:?}, y = {}", x, y);

    let m = 't';
    match m
    {
        'a' | 'b' => println!("its an a or b"),
        'c'..='m' => println!("c through to m"), // ''..'' without = is not yet allowd
        'n'..='t' => println!("n through to t"),
        _ => println!("Everything else")
    }

    let p = Point {x : 4, y : 7};
    
    let Point {x:a, y:b} = p;
    let Point {x, y} = p;

    println!("a = {}, b = {}", a, b);
    println!("x = {}, y = {}", x, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(Color::Hsv(45, 192, 0));
    match msg 
    {
        Message::Quit => println!("Asked to quit"),
        Message::Move {x, y} => println!("Move {} to the right, {} up", x, y),
        Message::Write(s) => println!("Print {} to screen", s),
        Message::ChangeColor(color) =>
        {
            match color
            {
                Color::Hsv(x, y, z) => println!("Color hsv: {}, {}, {}", x, y, z),
                Color::Rgb(r, g, b) => println!("Color rgb: {}, {}, {}", r, g, b)
            }
        }
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3,4);
    /* Alternative match
    match msg 
    {
        Message::Quit => println!("Asked to quit"),
        Message::Move {x, y} => println!("Move {} to the right, {} up", x, y),
        Message::Write(s) => println!("Print {} to screen", s),
        Message::ChangeColor(Color::Hsv(x, y, z)) => println!("Color hsv: {}, {}, {}", x, y, z),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Color rgb: {}, {}, {}", r, g, b)
    }*/

    let s = Some(String::from("Hello!"));
    // _ doesn't bind or move but _x would move it
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let num = Some(4);

    match num
    {
        Some(x) if x > 10 => println!("Greater than 10: {}", x),
        Some(x) => println!("Less than or equal to 10: {}", x),
        _ => ()
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg2 = Message::Move{x: 8, y: 16};
    match msg2
    {
        Message::Move{x, y: b_vab @3..=6} => println!("y is within 3..6 range :{} x is {}, y is {}", b_vab, x, y),
        Message::Move{x: 5..=10, y} => println!("x is with 5 to 10 range:{}, y is {}",x, y),
        Message::Move{x, y} => println!("Neither x nor y falls within range: {} and {}", x, y),
        _ => println!("Nothing to see here")
    }
}

struct Point
{
    x: i32,
    y: i32
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

// useful when implementing traits, otherwise remove unneeded param
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
