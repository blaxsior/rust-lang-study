use std::fmt::Display;

mod aggregator;
fn main() {
    println!("Hello, world!");
    // 제네릭 => 제네릭 파라미터에 타입 넣어 대체
    {
        let number_list = vec![1, 2, 3, 4, 5];
        // let largest_number = largest(&number_list);
        // println!("largest = {largest_number}");
    }

    // 제네릭 구조체
    {
        let int_point = Point { x: 3, y: 4 };
        let float_point = Point { x: 1.5, y: 4.3 };
    }

    // 트레잇
    {
        use aggregator::{NewsArticle, Tweet};

        let tweet = Tweet {
            username: String::from("popo"),
            content: String::from("포포는 잠이 좋아"),
            reply: false,
            retweet: false,
        };

        let news_article = NewsArticle {
            author: String::from("iseol"),
            content: String::from("나는 행복합니다"),
            headline: String::from("월급 200% 인상 소식..."),
            location: String::from("서울, 대한민국"),
        };
    }

    // 라이프타임
    {
        // let r;
        // {
        //     let x = 5;
        //     r = &x; // 이거 안됨
        // }
        // println!("r: {}", r);
    }

    // 함수의 라이프타임
    {
        let string1 = String::from("this is long string");
        let result;
        {
            let string2 = String::from("short string");

            result = longgest(&string1, &string2);
            println!("longgest {result}");
        }
    }
    // 구조체와 라이프타임
    {
        let mystruct;
        {
            let name = String::from("hello");
            mystruct = MyStruct { name: &name };
            mystruct.say_hello();
        }
    }

    {
        let my_ref;
        {
            let str_literal: &'static str = "Hello? I am str_literal";
            my_ref = str_literal;
        }
        println!("myref {my_ref}");
    }

    {
        let my_ref: &'static i32;
        {
            static NUMBER: i32 = 10;
            my_ref = &NUMBER;
        }
        println!("myref {my_ref}");
    }
}

// fn largest<T>(number_list: &[T]) -> &T {
//     let mut largest = &number_list[0];

//     for number in number_list {
//         if largest < number {
//             largest = number;
//         }
//     }

//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn hello() {
        println!("hello point!");
    }
}

impl Point<f64> {
    fn distance(&self, target: &Point<f64>) -> f64 {
        return ((self.x - target.x).powf(2.0) + (self.y - target.y).powf(2.0)).sqrt();
    }
}

trait Show {
    fn some_function(&self) -> String;
}

struct MyType;
// 내부 타입에 대해 내부 트레잇 구현 가능
impl Show for MyType {
    fn some_function(&self) -> String {
        String::from("i am MyType!")
    }
}
// 외부 타입에 대해 내부 트레잇 구현 가능
impl<T> Show for Vec<T> {
    fn some_function(&self) -> String {
        String::from("i am vector!")
    }
}
// // 외부 타입에 대해 외부 트레잇은 구현 불가
// impl<T> Display for Vec<T> {} // 에러 발생

fn longgest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct MyStruct<'a> {
    name: &'a str,
}

impl<'a> MyStruct<'a> {
    fn say_hello(&self) {
        println!("hello, my name is {}", self.name);
    }

    fn name(&self) -> &str {
        return self.name;
    }

    fn say_hello_to<'b>(&'a self, to: &'b str) -> &'a str {
        println!("hello, {to}");
        self.name
    }
}

fn get_first_name(full_name: &str) -> &str {
    return full_name.split(' ').next().unwrap_or(" ");
}

fn get_static() -> &'static str {
    "Hello"
}