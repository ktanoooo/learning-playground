fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1.area() is {:?}", rect1.area());

    // =======================================-
    // =======================================-
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // =======================================
    // =======================================
    let rect4 = Rectangle::square(3);
    println!("rect4 is {:?}", rect4);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数(selfを取らない関数)　instanced_a.method ではなく Struct::method() で呼び出す
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
