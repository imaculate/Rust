fn main() {
    let mut s = String::from("hello! world!");

    let index = index_first_word(&s);
    let first_word = first_word(&s);
    println!("Last Index of first word {}", index);

    println!("First word {}", first_word);

    s.clear(); // now index is no longer valid

    // uncommenting below leads to compile error sth about immutable then mutable borrow then use
    // println!("First word after clear {}", first_word);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word_generic(&my_string[..]);

    println!("First word using whole slice of String {}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_generic(&my_string_literal[..]); // holdup, how come can be reassigned?, because we shadowed it with let

    println!("First word using whole slice of string literal {}", word);


    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_generic(my_string_literal);

    println!("First word using just string literal {}", word);


}

fn index_first_word(s : &String) -> usize
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
    }
    s.len()
}

fn first_word(s : &String) -> &str
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_generic(s : &str) -> &str
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }

    &s[..]
}