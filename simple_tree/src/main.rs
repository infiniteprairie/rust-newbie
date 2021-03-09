//! This sample program, `simple_tree`, is taken from the Rust Book, 
//! chapter 15.6 (https://doc.rust-lang.org/book/ch15-06-reference-cycles.html) 
//! on smart pointers, strong and weak references
//! 
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new( Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("_leaf_ strong={}, weak={}", 
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new( Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);


        println!("inner scope...");

        println!("_branch_ strong={}, weak={}", 
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!("_leaf_ strong={}, weak={}", 
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        
    }

    println!("outer scope...");

    println!("_leaf_ strong={}, weak={}", 
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    
}
