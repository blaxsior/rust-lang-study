use rand::Rng;
use std::io::stdin;

fn main() {
    println!("숫자 맞추기 게임");
    println!("추측하는 숫자를 입력해보세요");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("입력을 받아오는데 실패했습니다.");

        let guessed_number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("숫자가 아닙니다");
                continue;
            }
        };
        println!("guess: {guess}");

        match guessed_number.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("to small"),
            std::cmp::Ordering::Equal => {
                println!("equal!");
                break;
            }
            std::cmp::Ordering::Greater => println!("to big"),
        }
    }
}

// let 변수는 이름이 같은 다른 변수로 덮어쓸 수 있음.
// let 자체는 변경 불가능. but 상수와는 달리 shadowing 가능
// const 변수는 변경이 불가능
