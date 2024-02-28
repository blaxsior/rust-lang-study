fn main() {
    // newtype
    {
        struct FullName(String);

        fn printName(name: &FullName) {
            println!("name: {}", name.0);
        }

        fn do_something(text: String) {
            //do something
        }

        let name = FullName(String::from("hong gil dong"));

        printName(&name); // 가능
                          // do_something(name); // 불가능
    }

    // type alias
    {
        type FullName = String;

        let text1 = String::from("hello");
        let name: FullName = String::from("hong gil dong");
        // 다른 이름으로 표현할 수 있을 뿐, 타입 자체는 동일.

        // 긴 타입을 짧은 이름으로 묶어 관리하기 쉬움
        type lambda = dyn Fn() + Send + 'static;
        let f: Box<lambda> = Box::new(|| println!("hello"));
    }
    // never type
    {
        fn run() -> ! {
            loop {
                //do_something
            }
        }
        
        fn error(message: &str) -> ! {
            panic!("{}", message);
        }
        
        let guess = String::from("some input");
        loop {
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        }
    }
}

