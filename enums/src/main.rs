#[derive(Debug)]
enum EnderecoIP {
    V4(u8, u8, u8, u8, u8),
    V6(String),
}

enum Mensagem {
    Escrever(String),
}

impl Mensagem {
    fn call(&self) {}
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let local: EnderecoIP = EnderecoIP::V4(127, 0, 0, 0, 1);
    let loopback: EnderecoIP = EnderecoIP::V6(String::from("::1"));

    println!("{:?} {:?}", local, loopback);  

    let msg: Mensagem = Mensagem::Escrever(String::from("Olá"));
    msg.call();

    let some_number: Option<i32> = Some(4);
    let some_text: Option<&str> = Some("text");

    let absent_number: Option<i32> = None;

    println!("{:?} {:?} {:?}", some_number, some_text, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum: i8 = x + y.unwrap_or(0);
    println!("{sum}");

    let lucky_coin: u32 = value_in_cents(Coin::Penny);
    println!("Lucky coin: {lucky_coin}");

    let coins: (Coin, Coin, Coin) = (Coin::Nickel, Coin::Dime, Coin::Quarter);
    println!("{:?}", coins);

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    println!("{:?} {:?}", six, none);
    

}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
