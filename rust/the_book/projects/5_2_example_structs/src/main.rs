fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("{}", area(width1, height1));

    let rect1 = (30, 50);
    println!("{}", area_refactor_taple(rect1));

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area_refactor_struct(&rect));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_refactor_taple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area_refactor_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
