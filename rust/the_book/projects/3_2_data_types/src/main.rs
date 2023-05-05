fn main() {
    // 複合型
    // 1. タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x : {}, y: {}, z: {}", x, y, z);

    println!("x: {}, y: {}, z: {}", tup.0, tup.1, tup.2);

    // 2. 配列型

    // 全て同じ型である必要がある
    let array = [1, 2, 3, 4, 5];
    println!("array: {}", array[0]);

    let array2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array2: {:?}", array2);

    let array3 = [3; 5];
    println!("array3: {:?}", array3);
}
