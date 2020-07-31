use std::collections::HashMap;

fn main() {
    // does not compile since v1 is immutable
    /*let v1 : Vec<i32> = Vec::new();
    v1.push(3);*/

    // ========================== vectors =================================
    let mut v2 = Vec::new();
    v2.push(9);
    v2.push(15);

    println!("The vector is {:?}", v2);

    let v3 : Vec<i32> = vec![5, 7, 12, 19, 31];

    let third = v3[2];
    println!("The third element is {}", third);

    match v3.get(6)
    {
        None => println!("There is no sixth element"),
        Some(t) => println!("The sixth element is {}", t)
    }

    let mut v4 = vec![1, 2, 3, 4];
    v4.push(7);
    let first = v4[0];
    v4.push(5);
    // below line should not work due to mutable/immutable references
    println!("The first element is: {}", first);

    let v5 = vec![100, 32, 57];
    for i in v5 {
        println!("{}", i);
    }

    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
        println!("{}", i);
    }

    let mut v7 = Vec::new();
    let v7s = String::from("Lazy");
    v7.push(v7s);
    // println!("{}", v7s); // vector has taken ownership of v7s

    // ========================== strings =================================
    let data = "Initial contents"; //&str
    let s0 = data.to_string();
    let mut s1 = "initial".to_string(); // similar to String::from("initial");
    let s2 = ": moved ownership? not with push_str";
    s1.push_str(" content");
    s1.push_str(s2);
    s1.push('!');
    println!("s1 is {}", s1);

    let s3 = s0 + s2 + data; // s2 can be coerced to be &str, also first parameter must be String not &str;
    println!("s3 is {}", s3); 
    // println!("Is s0 still valid? {}", s0); // no because concat took ownership of self
    println!("Is s2 still valid? {}", s2);
    println!("Is data still valid? {}", data);

    let s4 = format!("{}-{}-{}", s1, s2, data);
    println!("Format concatenated and didn't take any ownership: {}",s4);

    let hello = String::from("नमस्ते");
    println!("hello is {}", hello); // indexing is ambigous (byte? grapheme(visible chars)? char?)
    println!("Sliced {}", &hello[6..12]);

    let hello2 = String::from("hello");
    println!("First char {}", &hello2[0..1]);

    //char iterating, 6 chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //byte iterating .. 18 bytes
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

    // ========================== hashmaps =================================
    // create hashmap by insertion
    let mut scores = HashMap::new();
    scores.insert("Barcelona", 5);
    scores.insert("Madrid", 8);
    println!("Football scores {:?}", scores);

    let teams = vec!["ManU", "Everton"];
    let init_scores = vec![5, 18];
    // collect() necessiates annotation
    let score_map : HashMap<_, _> = teams.into_iter().zip(init_scores.into_iter()).collect();
    println!("Hashmap from vector {:?}", score_map);

    let field_key = String::from("Color");
    let field_value = String::from("Pink");
    let mut map = HashMap::new();
    map.insert(field_key, field_value);
    // println!("Is field key still usable? {}",field_key); // No , it is now owned by map

    let score = scores.get("Barcelona");
    println!("Score extracted: {}", score.expect("Barcelona hasn't played"));

    let fk = String::from("Color");
    let fv = map.get(&fk);
    println!("Field value extracted: {}", fv.expect("no color field"));

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    let mut map2 = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map2);
}
// Qn : do vectors take ownership?
