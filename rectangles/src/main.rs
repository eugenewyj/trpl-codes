fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("1:长方形面积是：{}", area(width1, height1));

    let rect1 = (30, 50);
    println!("2:长方形面积是：{}", area_tuple(rect1));

    let rect2 = Rectangle{width: 30, height: 50};
    println!("3:长方形面积是：{}", area_struct(&rect2));
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
