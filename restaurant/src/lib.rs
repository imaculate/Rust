mod front_of_house;

mod back_of_the_house
{
    #[derive(Debug)]
    pub struct Breakfast
    {   
        pub bread : String,
        fruit : String
    }

    impl Breakfast
    {
        pub fn summer(bread: String) -> Breakfast
        {
            Breakfast{
                bread,
                fruit: String::from("Kiwi")
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer
    {
        Soup,
        Salad
    }

    fn fix_incorrect_order()
    {
        println!("Fixing incorrect order");
        cook_order();
        crate::front_of_house::serving::serve_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order()
    {
        println!("Cooking order");
    }
}

pub use front_of_house::hosting; // remember why pub use
use back_of_the_house::Breakfast as Meal; // full path to Enum/Struct prefered unless there is conflict, where alias is allowed
use self::back_of_the_house::Appetizer; // relative paths allowed

// alternative imports
// use back_of_the_house::{Breakfast, Appetizer};
// use back_of_the_house::*;
// use back_of_the_house::{self, Breakfast};

fn eat_at_restaurant()
{
    hosting::add_to_waitlist();
    let mut meal = Meal::summer(String::from("Rye"));
    meal.bread = String::from("Scones");
    println!("I am having {:?}", meal);
    // meal.fruit = String::from("Berries"); doesn't compile

    // below doesn't work because fruit is not set and can't be because it is private
    /*let meal2  = back_of_the_house::Breakfast{
        bread: String::from("Baquette"),
    };*/

    let app = Appetizer::Salad;
    println!("I am having appetizer {:?}", app);
}