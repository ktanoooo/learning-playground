fn main() {
    if_fn();
    println!("===========================");
    loop_fn();
    println!("===========================");
    while_fn();
    println!("===========================");
    while_array_fn();
    println!("===========================");
    for_fn();
}

fn if_fn() {
    let x = 3;

    if x < 5 {
        println!("condition was true");
    } else if x == 5 {
        println!("condition was equal");
    } else {
        println!("condition was false");
    }

    // bool以外はif文には使えない。例えば zero 判定は次のようにする
    if x != 0 {
        println!("x is not zero");
    }

    let y = if x == 3 { 5 } else { 6 };
    println!("y is: {}", y);
}

fn loop_fn() {
    let mut count = 0;
    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {}", count);
}

fn while_fn() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_array_fn() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("value: {}", a[index]);

        index += 1;
    }
}

fn for_fn() {
    let a = [10, 20, 30, 40, 50];

    for e in a {
        println!("value: {}", e);
    }
}
