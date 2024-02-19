fn main() {
    println!("Hello, world!");

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    let v4 = IpAddrKind::V4(1, 1, 1, 1);
    let v6 = IpAddrKind::V6("::1");

    if let IpAddrKind::V4(a, b, c, d) = v4 {
        println!("v4, {a} {b} {c} {d}");
    }

    let value1 = Some(4);
    match value1 {
        Some(v) => println!("{v}"),
        None => println!("this is None"),
    }

    // 나이의 1의자리 숫자가 3의 배수인 경우를 체크
    let age = 10;
    match age % 10 {
        3 | 6 | 9 => println!("나이 뒷자리가 3, 6, 9가 포함됨"),
        other => println!("나이 = {other}"),
    };

    // 숫자가 1 또는 2라면 당첨
    let dice = 3; 
    match dice {
        1 | 2 => println!("당첨!"),
        _ => println!("당첨되지 않음...")
    }

    let coin = Coin::Penny;
    if let Coin::Penny = coin {
        println!("this is penny")
    }

    let opt_val = Some(13);
    
    if let Some(x) = opt_val {
        println!("{}", x);
    }

}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(&'static str),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Self::Quit => println!("quit"),
            Self::Move { x, y } => println!("move by {x} {y}"),
            Self::Write(string) => println!("write {string}"),
            Self::ChangeColor(r, g, b) => println!("change color {r} {g} {b}"),
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
