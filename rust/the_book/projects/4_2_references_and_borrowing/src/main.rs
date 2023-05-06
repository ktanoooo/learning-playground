fn main() {
    references_fn();
    change_referecnes_fn();
    change_references_ok_fn();

    let _da = dangle2();
}

fn references_fn() {
    let s1 = String::from("hello");

    let len = _calc_length(&s1);

    println!("s1: {}, len: {}", s1, len);
}

fn _calc_length(s: &String) -> usize {
    s.len()
}

fn change_referecnes_fn() {
    let s2 = String::from("hello");
    _change_string(&s2);

    println!("s: {}", s2);
}

fn _change_string(_s: &String) {
    // _s.ppush_str(", world!");  // sは参照元がimutableなのでコンパイルエラー
}

fn change_references_ok_fn() {
    let mut s3 = String::from("hellow");
    _change_string_ok(&mut s3);

    println!("s: {}", s3);
}

fn _change_string_ok(s: &mut String) {
    s.push_str(", world!");
}

// 特定のスコープで、ある特定のデータに対しては、 一つしか可変な参照を持てない
// let mut s = String::from("hello");
// let r1 = &mut s;
// let r2 = &mut s;

// dangle() 実行後、参照が返されるが、スコープを抜けるとドロップされるのでコンパイルエラー
// fn dangle1() -> &String {
//     let s = String::from("hellow");
//     &s
// }
// 直接返す
fn dangle2() -> String {
    let s = String::from("hellow");
    s
}
