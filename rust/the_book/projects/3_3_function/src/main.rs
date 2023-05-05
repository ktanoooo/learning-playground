fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_arg(3);

    let x = five();
    println!("five is: {}", x);
}

fn another_function() {
    println!("another_function");
}

fn another_function_with_arg(x: i32) {
    println!("another_function_with_arg is: {}", x);
}

// 式と文
// 文は値を返さない。式は値を返す。
// 文は式で構成されている。
// 文の末尾にはセミコロンをつける。式にはつけない。
// 文の末尾にセミコロンをつけると式になる。

// 式の関数には返り値の型を示す
fn five() -> i32 {
    5 // <- 式なので5が返る
}

// 文なのに返り値の型を示しているのでコンパイルエラー
fn seven() {
    7; // <- 文なので値が返らない。 let x = seven(); で7にはならない
}
