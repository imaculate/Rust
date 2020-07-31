pub fn add_two(i: u32) -> u32
{
    println!("I got a value of {}", i);
    i + 2
}

pub struct Guess
{
    value: u32
}

impl Guess
{
    pub fn new(value : u32) -> Guess
    {
        if (value < 1) || (value > 100) 
        {
            panic!("Guess must be a value less than 1 and greater than 100");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_adds_two()
    {
        let x = 2;
        let expected = 4;
        let result = add_two(x);
        assert_eq!(expected, result, "Add two to {} resulted in {} instead of {}", x, result, expected);
    }

    /*#[test]
    fn test_that_fails()
    {
        panic!("You shall not pass!");
    }*/

    #[test]
    #[ignore]
    #[should_panic(expected = "Guess must be a value less than 1 and greater than 100")]
    fn test_guess_panics()
    {
        Guess::new(200);
    }

    #[test]
    fn returns_err() -> Result<(), String>
    {
        if add_two(2) == 4
        {
            Ok(())
        }
        else
        {
            Err(String::from("add_two to 2 didn't result in 4"))
        }
    }
}
