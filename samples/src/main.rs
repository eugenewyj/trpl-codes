fn main() {
    ch3_27();
    ch6_28();
    ch6_29();
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
    let c = || { env_var + 2 };
    assert_eq!(3, c());
}

/// 闭包示例，trait对象
fn ch6_29() {
    let env_var = 1;
    let c: Box<dyn Fn() -> u32> = Box::new(|| { env_var + 2 });
    assert_eq!(3, c());
}