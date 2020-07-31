use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node
{
    value: i32,
    parent: RefCell<Weak<Node>>,
    childen: RefCell<Vec<Rc<Node>>>
}

fn main() {
    let leaf = Rc::new(
        Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            childen: RefCell::new(vec![])
        }
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
    println!("Entering inner...");
    {
        let branch = Rc::new(
            Node{
                value: 7,
                parent: RefCell::new(Weak::new()),
                childen: RefCell::new(vec![Rc::clone(&leaf)])
            }
        );

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        println!("branch strong count = {}, weak count = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    }
    println!("Leaving inner...");

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
