use std::ops::Drop;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
        println!("drop for {}", self.0);
    }
}

fn test1() {
    println!("test1 ....");
    let x = S(1);
    println!("create x: {:?}", x);
    let x = S(2);
    println!("create shadowing X: {:?}", x);
}

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    data: T,
    next: NodePtr<T>,
}
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn test2() {
    println!("test2 ....");
    let first = Rc::new(RefCell::new(Node { data: 1, next: None }));
    let second = Rc::new(RefCell::new(Node { data: 2, next: None }));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first.clone());
}

struct A {
    a: u32,
    b: Box<u64>,
}
struct B(i32, f64, char);
struct N;
enum E {
    H(u32),
    M(Box<u32>)
}
union U {
    u: u32,
    v: u64
}

fn test3() {
    println!("test3 ....");

    println!("Box<u32>: {:?}", std::mem::size_of::<Box<u32>>());
    println!("A: {:?}", std::mem::size_of::<A>());
    println!("B: {:?}", std::mem::size_of::<B>());
    println!("N: {:?}", std::mem::size_of::<N>());
    println!("E: {:?}", std::mem::size_of::<E>());
    println!("U: {:?}", std::mem::size_of::<U>());
}

fn main() {
    test1();    
    test2();
    test3();
}