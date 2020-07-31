fn main() {
    let p1 = CustomSmartPointer
    {
        data: String::from("My stuff")
    };

    let p2 = CustomSmartPointer
    {
        data: String::from("Other stuff")
    };

    println!("Created custom smart pointers");
    // explicit drop not allowed
    //p1.drop();
    drop(p1);
    println!("Dropped p1 before out of scope");
}

struct CustomSmartPointer
{
    data: String
}

impl Drop for CustomSmartPointer
{
    fn drop(&mut self)
    {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}
