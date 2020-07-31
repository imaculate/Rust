#[derive(Debug)]
enum UsState
{
    Alaska,
    Alabama,
    Arizona,
    California,
    Colorado,
    Idaho,
    Georgia,
    Florida,
    Hawaii,
    Indiana,
    Iowa,
    MaryLand,
    Massachussetts,
    Minnesota,
    Montana,
    NewYork,
    NorthCarolina,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylavia,
    SouthCarolina,
    Tennessee,
    Texas,
    WestVirginia,
    Washington
}

#[derive(Debug)]
enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    // Illustrating enum depth
    let coins = [Coin::Quarter(UsState::Texas), Coin::Dime, Coin::Nickel, Coin::Quarter(UsState::Washington), Coin::Penny];

    let mut sum = 0;
    for coin in coins.iter()
    {
        let v = value_of_coin(coin);
        println!("The value of {:?} is {}", coin, v);
        sum += v;
    }

    println!("The sum of the coins is {}", sum);

    // Illustrating null
    let nums = [None, Some(3), Some(11)];
    for num in nums.iter()
    {
        println!("{:?} Added one: {:?}", num, add_one(num));
    }

    //Illustrating placeholder
    let some_u8s = [8, 3, 7, 15, 16, 98];
    for some_u8 in some_u8s.iter()
    {
        match some_u8 
        {
            1 => println!("One"),
            7 => println!("Seven"),
            16 => println!("Sixteen"),
            _ => println!("Nothing special") //or no-op ()
        }

        // Illustrating if let
        if let 7 = some_u8
        {
            println!("If let detected a 7");
        }
        else
        {
            println!("If let detected nothing of value");
        }
    }
}

fn value_of_coin(coin: &Coin) -> u8
{
    match coin 
    {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>
        {
            println!("Quarter from {:?}", state);
            25
        }
    }
}

fn add_one(x: &Option<i8>) -> Option<i8>
{
    match x 
    {
        None => None,
        Some(i) => Some(i+1)
    }
}
