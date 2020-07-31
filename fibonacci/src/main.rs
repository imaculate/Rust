use std::io;

fn main() {
    println!("Enter a number between 0 and 30: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line!");
    let n: i32 = n.trim().parse().expect("Expected a number!");

    let fib = fibonacci(n);
    println!("The {}th fibonacci is {}", n, fib);
}

fn fibonacci(n : i32) -> i32
{
    if (n > 30) || (n < 0)
    {
        println!("Out of range number, must be between 0 and 30");
        return -1;
    }

    if n < 2
    {
        return n;
    }
    
    let mut n0 = 0;
    let mut n1 = 1;
    for _number in 2..(n+1) {
        n1 = n0 + n1;
        n0 = n1 - n0;
    }

    return n1;
}