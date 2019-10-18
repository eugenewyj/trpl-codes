use std::cell::RefCell;
use std::ops::Drop;
use std::rc::Rc;

fn main() {
    ch3_27();
    ch6_28();
    ch6_29();

    let x = 1_i32;
    x.foo2();

    let p = &x as &dyn Foo;
    p.foo1();

    let x = S(1);
    println!("create x: {:?}", x);
    let x = S(2);
    println!("create shadowing x: {:?}", x);

    test_node();
}

#[derive(Debug)]
struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
        println!("drop for {}", self.0);
    }
}

trait Foo {
    fn foo1(&self);
    fn foo2(&self)
    where
        Self: Sized;
}

impl Foo for i32 {
    fn foo1(&self) {
        println!("foo1 {}", self);
    }

    fn foo2(&self) {
        println!("foo2 {}", self);
    }
}

/// 加法trait采用关联类型
trait Add<RHS = Self> {
    type Output;
    fn add(self, other: RHS) -> Self::Output;
}

/// 实现i32 + u32 -> i32
impl Add<u32> for i32 {
    type Output = i32;
    fn add(self, other: u32) -> i32 {
        (self + other as i32) as i32
    }
}

/// 验证实现的i32 + u32 -> i32方法。
fn ch3_27() {
    assert_eq!(3i32, 1i32.add(2u32));
}

/// 闭包示例代码
fn ch6_28() {
    let env_var = 1;
    let c = || env_var + 2;
    assert_eq!(3, c());
}

/// 闭包示例，trait对象
fn ch6_29() {
    let env_var = 1;
    let c: Box<dyn Fn() -> u32> = Box::new(|| env_var + 2);
    assert_eq!(3, c());
}

///循环应用示例
type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> where T: std::fmt::Debug {
    data: T,
    next: NodePtr<T>,
}
impl<T> Drop for Node<T> where T: std::fmt::Debug {
    fn drop(&mut self) {
        println!("Dropping Node data= {:?}", self.data);
    }
}

fn test_node() {
    let first = Rc::new(RefCell::new(Node {
        data: 1,
        next: None,
    }));
    let second = Rc::new(RefCell::new(Node {
        data: 2,
        next: None,
    }));
    first.borrow_mut().next = Some(second.clone());
    // second.borrow_mut().next = Some(first.clone());
}
