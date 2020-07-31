#[derive(Debug)]
enum Message
{
    Quit,
    Move (i32, i32, i32),
    Write(String),
    ChangeColor {r: u32, g: u32, b: u32},
    HitCount(Frequency)
}

#[derive(Debug)]
struct Frequency
{
    sessions : u32,
    users: u32
}

impl Message
{
    fn call(&self)
    {
        println!{"The message is {:#?}", self}
    }
}

fn main() {
    let msg1 = Message::Write(String::from("Sent from a farway land"));
    let msg2 = Message::Quit;
    let msg3 = Message::Move(5, -6, 4);
    let msg4 = Message::ChangeColor{ r: 0, g: 57, b: 255};

    let freq  = Frequency
    {
        sessions: 45,
        users: 34
    };
    let msg5 = Message::HitCount(freq);

    let messages = [msg1, msg2, msg3, msg4, msg5];

    for msg in messages.iter()
    {
        msg.call();
    }

    // Null Options
    let mut num = 5;
    let some_num : Option<i8> = Some(3);
    let absent_num : Option<i8> = None;
    
    let option_nums = [some_num, absent_num];
    for o_num in option_nums.iter()
    {
        if o_num.is_some()
        {
            num = num + o_num.expect("This is none, what are you thinking?");
        }
    }

    println!("Num is now {}", num);
    
    // let number = if condition { 5 } else { 6 };
}
