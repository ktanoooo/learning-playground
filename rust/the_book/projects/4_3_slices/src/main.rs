fn main() {
    slice_fn();
    slice_fn_slice();
}

fn slice_fn() {
    let mut s = String::from("hello world");

    // 可変参照sを渡しても、そのあとsはclear()されて、空("")になる。
    // その後のwordはs("")に対して無効な数値を維持しており、これでコンパイルが通るのでバグの原因になる。 -> そこでスライス
    let word = first_word_len(&s);
    s.clear();

    println!("the first word is: {}", word);
}

fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);
    println!("iter: {:?}", bytes.iter());
    println!("enumerate: {:?}", bytes.iter().enumerate());

    // enumerate() はコレクションの要素を返す
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' はバイトリテラル
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn slice_fn_slice() {
    let mut s = String::from("hello world");

    // スライスは部分的な参照であり、この関数は不変な参照 &s[..] (-> &str)を返すため、その後の可変な参照`s.clear()`は借用規則に反してコンパイルエラーとなる
    let word = first_word_len_slice(&s);

    s.clear();

    println!("the first word is: {}", word);
}

fn first_word_len_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);
    println!("iter: {:?}", bytes.iter());
    println!("enumerate: {:?}", bytes.iter().enumerate());

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
