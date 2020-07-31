use std::fmt::Display;

fn main() 
{
    let p1 = Point
    { 
        x: 1.7,
        y: 6.2
    };

    println!("p1.x = {}", p1.x());
    println!("distance from origin = {}", p1.dist_from_origin());
    p1.cmp_display();

    let garbage = ["dfd", "adwe", "qwqwq", "qwdwegyh"];
    println!("The longest element in the list is {}", longest(&garbage));

    let tweet = Tweet
    {
        username: String::from("Culey"),
        content: String::from("Regular scheduled programming"),
        reply: false,
        retweet: true
    };
    println!("1 new tweet {}", tweet.summarize());

    let article = NewsArticle
    {
        headline: String::from("Psychological impacts of Covid-19"),
        location: String::from("Auburn, WA"),
        author: String::from("Marie Forleo"),
        content: String::from("Even if you don't get affected, Covid-19 may leave your psych permanently damaged")
    };

    // println!("New article alert {}", article.summarize());
    notify(&article, &tweet);

    notify2(&article, &tweet);

    let s1 = String::from("Hopeful longest");
    let result;
    {
        let s2 = String::from("Competitive longest");
        result = longest_string(s1.as_str(), s2.as_str());
        println!("Longest string is {}", result);
    }
    // this wont compile because s2 doesn't live long enough
    // println!("Longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentense = novel.split('.').next().expect("Could not find .");
    let i = ImportantExcerpt
    {
        part: first_sentense
    };

    println!("Important Excerpt: {}", i.make_announcement("Some things are best not announced"));
    // static lifetimes lives throughout the program, may not necessaily be the best workaround
    let s: &'static str = "I have a static lifetime."
}

fn longest<T : PartialOrd>(list: &[T]) -> &T
{
    let mut result = &list[0];

    for item in list
    {
        if item > result
        {
            result = item;
        }
    }
    result
}

// Lifetimes are about connecting the lifetimes of various parameters and return values of functions
fn longest_string<'a>(x : &'a str, y : &'a str) -> &'a str
{
    if x.len() > y.len()
    {
        x
    }
    else
    {
        y
    }
}

fn notify(item1: &impl Summary, item2: &impl Summary)
{
    println!("Breaking news! {} ... {}", item1.summarize(), item2.summarize())
}

fn notify2<T, U>(item1: &T, item2: &U)
    where T : Summary, U: Summary
{
    println!("Breaking news! {} ... {}", item1.summarize(), item2.summarize())
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
{
    fn x(&self) -> &T
    {
        &self.x
    }
}

impl Point<f32>
{
    fn dist_from_origin(&self) -> f32
    {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Display + PartialOrd> Point<T>
{
    fn cmp_display(&self)
    {
        if self.x > self.y
        {
            println!("The largest member is x = {}", self.x);
        }
        else
        {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub trait Summary
{
    fn summarize_author(&self) -> String; 
    fn summarize(&self) -> String
    {
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle
{
    fn summarize_author(&self) -> String
    {
        self.author.clone()
    }

    fn summarize(&self) -> String
    {
        format!("{}, by  {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet
{
    fn summarize_author(&self) -> String
    {
        format!("@{}", self.username)
    }
}

// lifetimes of references in structs =>the struct can't outlive the reference

struct ImportantExcerpt<'a>
{
    part: &'a str
}

impl<'a> ImportantExcerpt<'a>
{
    // no need to declare lifetimes in these methods due to lifetime elision rules
    fn make_announcement(&self, announcement: &str) -> &str
    {
        println!("Attention please: {}", announcement);
        self.part
    }

    fn level(&self) -> i32
    {
        3
    }
}

fn longest_and_announcement<'a, T>(x : &'a str, y: &'a str, ann: T) -> &'a str 
where T:Display
{
    println!("Attention please: {}", ann);
    if x.len() > y.len()
    {
        x
    }
    else
    {
        y
    }
}