use std::ops::{Deref, DerefMut};
use std::fmt::Debug;
use std::rc::Rc;
use crate::List::{Cons, Nil};

// we can use box to construct a Cons list
#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn peek(&self) -> Option<&T> {
        match self {
            List::Cons(v, _) => Some(v),
            List::Nil => None,
        }
    }
    
    pub fn recurse(&self) -> Rc<List<T>> {
        match self {
            List::Cons(_, p) => Rc::clone(&p), // clone the Rc
            List::Nil => Nil.into(), // into makes return type just work
        }
    }
}    

#[derive(Debug)]
struct Boxlike<T: Debug>(T);

impl<T: Debug> Boxlike<T> {
    pub fn new(x: T) -> Boxlike<T> {
        Boxlike(x)
    }
}

/* The deref method gives the compiler the ability to take a value of 
 * any type that implements Deref and call the deref method to get a & 
 * reference that it knows how to dereference.
 */
impl<T: Debug> Deref for Boxlike<T> {
    type Target = T; // associated type
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> DerefMut for Boxlike<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> Drop for Boxlike<T> {
    fn drop(&mut self) {
        println!("Dropping Boxlike with data `{:?}`!", self.0);
    }
}

fn main() {
    /* Box is the most basic example of smart pointer. It implements both
     * the Deref and Drop trait. It qualifies as a "recursive" type, and
     * its contents are stored on the heap.
     */
    let mut n = 1;
    let b = Box::new(&mut n);
    **b += 1;
    println!("Was 1 incremented? {}", n > 1);
    
    let list = Rc::new(
                    Cons(1, 
                        Rc::new(
                            Cons(2, 
                                Rc::new(
                                    Cons(3, 
                                        Rc::new(Nil)))))));
    
    let mut head = Rc::clone(&list);
    
    while let Some(value) = head.peek() {
        println!("Peeked list value: {:?}", value);
        head = head.recurse();
    }
    
    let b = Boxlike::new("Hello");
    println!("{}", *b);
    let b = Boxlike::new(String::from("World"));
    println!("{}", *b);
    
    let mut n = 1;
    let mut b = Boxlike::new(&mut n);
    **b += 1;
    drop(b); /* drop needed because Drop trait implemented
    error otherwise because n is mutably borrowed */
    println!("Was 1 incremented? {}", n > 1);
    
}
