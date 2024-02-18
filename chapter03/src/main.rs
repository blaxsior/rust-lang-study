fn main() {
    let mut x = 5;
    println!("value x = {x}");
    x = 6;
    println!("value x = {x}");

    const value1: i32 = 1;

    let guess: i32 = "42".parse().expect("Not a number!");
    let guess = "42".parse::<i32>().expect("Not a number!");

    let fval1 = 1.3;
    let fval2: f32 = 1.2;

    let tuple = ("hello", 'C', 13);
    let (hello, ch, num) = tuple;
    println!("{}", tuple.0);
    println!("{}", tuple.1);

    let myunit = ();

    let (unit) = myunit;

    println!("{:?}", unit);

    let array = [1, 2, 3, 4];
    let array = [5; 4]; // 5를 4개 가진 배열
    let value = helloFunc();
    println!("{value}");

    let mut number = 0;

    let value = loop {
        number += 1;
        if number > 10 {
            break number;
        }
    };

    let number = 3;

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let score = 70;

    let grade = if score < 50 {
        'C'
    } else if score < 70 {
        'B'
    } else {
        'A'
    };

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("count in end = {count}");


    let numbers = [1,2,3,4,5];

    for number in numbers {
        println!("{number}");
    }

    for i in 1..10 {
        println!("{}", fibonacci(i));
    }
}

fn helloFunc() -> i32 {
    5
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let (mut a, mut b) = (0, 1);

    for i in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
