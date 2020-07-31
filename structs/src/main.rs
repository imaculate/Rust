#[derive(Debug)]
struct User
{
    email: String, // owned member, reference has to have lifetime
    username: String,
    sign_in_count: u64,
    active: bool
}

// tuple structs , when the order of members is obvious
#[derive(Debug)]
struct Color(i32, i32, i32);

// unit like structs useful when associated with traits
// struct Unit; 

fn main() {
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1 
    };

    println!("User1 is {:?}", user1);
    user1.email = String::from("anotherone@example.com");

    println!("Email changed, User1 is now {:?}", user1);

    let user2 = build_user(String::from("rand@gmail.com"), String::from("rand"));
    println!("Built from strings, User2 is {:?}", user2);

    let user3 = build_user_from_user(user2);
    println!("Built from user2, User3 is {:?}", user3);

    let user3 = build_user_from_user(user1);
    println!("Built from user1, User3 is {:?}", user3);

    // Both user1 and user2 are no longer usable
    // println!("Is user1 usable? {:?}", user1);
    // println!("Is user2 usable? {:?}", user2);

    let green = Color(0, 1, 0);
    println!("Green is {:?}", green);
}

fn build_user(email: String, username: String) -> User
{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn build_user_from_user(user1 : User) -> User
{
    User{
        email: String::from("anonymous@example.com"),
        username: String::from("anonymous"),
        ..user1
    }
}
