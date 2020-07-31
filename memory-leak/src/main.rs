use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List
{
    Nil,
    Cons(i32, RefCell<Rc<List>>)
}

impl List
{
    fn tail(&self) -> Option<&RefCell<Rc<List>>>
    {
        match self 
        {
            Nil => None,
            Cons(_, item) => Some(item),
        }
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial Rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(6, RefCell::new(Rc::clone(&a))));
    println!("b initial Rc count = {}", Rc::strong_count(&b));
    println!("a initial Rc count after b init = {}", Rc::strong_count(&a));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail()
    {
        *(link.borrow_mut()) = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());
    //println!("a = {:?}", a);
}