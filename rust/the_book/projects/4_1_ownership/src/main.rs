fn main() {
    string_fn();
    heap_copy();
    stack_copy();
    clone_fn();
    ownership_fn();
}

fn string_fn() {
    let mut s = String::from("hello");
    println!("s: {}", s);

    s.push_str(", world!");
    println!("s: {}", s);
}

fn heap_copy() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // s1はs2にmoveされているのでコンパイルエラー
}

fn stack_copy() {
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
}

fn clone_fn() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}

fn ownership_fn() {
    let s = String::from("hello");
    // sの所有権がムーブする
    ownership_1(s);
    // println!("s: {}", s); // sは所有権が移動しているのでコンパイルエラー

    let x = 5;
    ownership_2(x); // xはCopyなのでムーブしない
    println!("x: {}", x); // xはCopyなのでムーブしていない
}

fn ownership_1(arg: String) {
    println!("arg: {}", arg);
}

fn ownership_2(arg: i32) {
    println!("arg: {}", arg);
}
