fn main() {
    println!("Hello, world!");

    let user = User {
        active: true,
        username: String::from("blaxsior"),
        email: String::from("hello"),
        sign_in_count: 0,
    };

    // 일종의 spread operator처럼 나머지 값 채우기 가능
    // ..으로 값을 채우면 기존의 할당 방식처럼 copy 또는 move처리되므로 주의!
    let user2 = User {
        username: String::from("cocopam"),
        email: String::from("world"),
        ..user
    };

    let rgb = RGB(0, 0, 0);
    let pos2d = Vec2D(10, 3);

    let animal = AnyAnimal;

    let rectangle = Rectangle {
        width: 13,
        height: 42,
    };
    println!("{:?}", rectangle); // 일반적으로 출력
    println!("{:#?}", rectangle); // 예쁘게 출력

    println!("{}", area(&rectangle));

    let rectangle2 = Rectangle::new(32, 44);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new(active: bool, username: String, email: String) -> Self {
        Self {
            active,
            username,
            email,
            sign_in_count: 0,
        }
    }
}

impl User {
    fn setActive(&mut self, active: bool) {
        self.active = active;
    }

    fn inc_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }
}

#[derive(Debug)] // 디버깅 정보를 출력할 수 있게 도와주는 트레잇
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(r: &Rectangle) -> u32 {
    return r.width * r.height;
}

struct RGB(i32, i32, i32);
struct Vec2D(i32, i32);

struct AnyAnimal;
