#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // ユニット構造体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32); // タプル構造体

impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


fn main() {

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}", home);

    println!("loopback: {:?}", loopback);

    let m = Message::Write(String::from("hello"));

    m.call();

    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("value: {}", value);

    let coin = Coin::Nickel;
    let value = value_in_cents(coin);
    println!("value: {}", value);


}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
