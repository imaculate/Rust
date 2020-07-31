use std::cell::RefCell;
#[derive(Debug)]
enum List
{
    Cons(i32, Box<List>),
    Nil
}

#[derive(Debug)]
enum RcList
{
    RcCons(i32, Rc<RcList>),
    RcNil
}

#[derive(Debug)]
enum RfcList
{
    RfcCons(Rc<RefCell<i32>>, Rc<RfcList>),
    RfcNil
}

use std::rc::Rc;
use crate::List::{Cons,Nil};
use crate::RcList::{RcCons,RcNil};
use crate::RfcList::{RfcCons, RfcNil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Cons list is {:?}", list);

    let x = 5;
    let y = &x;

    println!("Reference y is {}", y);

    // below wont compile because can't compare integer to {integer}
    //assert_eq!(5, y);
    assert_eq!(5, *y);

    let b = Box::new(x);
    assert_eq!(5, *b);

    let mb = MyBox::new(x);
    assert_eq!(5, *mb);

    let bn = MyBox::new(String::from("Maya"));
    // Instead of this
    greeting(&(*bn)[..]);
    // we can do this due to deref coercion, no runtime costs, coercion performed at compile time
    greeting(&bn);

    //reference counting
    let mid_list = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let _a_list = Cons(3, Box::new(mid_list));
    // can't re-use mid_list since it is now owned by a_list
    //let b_list = Cons(4, Box::new(mid_list));

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("Count after creating: {}", Rc::strong_count(&a));
    let _b = RcCons(3, Rc::clone(&a));
    println!("Count after linking to b: {}", Rc::strong_count(&a));
    let _c = RcCons(4, Rc::clone(&a));
    println!("Count after linking to c: {}", Rc::strong_count(&a));
    {
        let _d = RcCons(4, Rc::clone(&a));
        println!("Count after linking to d: {}", Rc::strong_count(&a));
    }
    println!("Count after d goes out of scope: {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));
    let list1 = Rc::new(RfcCons(Rc::clone(&value), Rc::new(RfcNil)));
    let list2 = Rc::new(RfcCons(Rc::new(RefCell::new(6)), Rc::clone(&list1)));
    let list3 = Rc::new(RfcCons(Rc::new(RefCell::new(8)), Rc::clone(&list1)));

    *value.borrow_mut() += 10;

    println!("list1 after = {:?}", list1);
    println!("list1 after = {:?}", list2);
    println!("list1 after = {:?}", list3);
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T>
{
    fn new(x: T) -> MyBox<T>
    {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
{
    type Target = T;

    fn deref(&self) -> &T
    {
        &self.0
    }
}

fn greeting(name: &str)
{
    println!("Hello! {}", name);
}