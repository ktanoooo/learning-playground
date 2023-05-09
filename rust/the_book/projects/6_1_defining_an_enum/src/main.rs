fn main() {
    instanced_enum();

    enum_struct();

    enum_with_string();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn instanced_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:?}", four);
    println!("six: {:?}", six);
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn enum_struct() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}

#[derive(Debug)]
enum IpAddrKindWithAddr {
    V4(String),
    V6(String),
}

fn enum_with_string() {
    let home = IpAddrKindWithAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindWithAddr::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {}
}
