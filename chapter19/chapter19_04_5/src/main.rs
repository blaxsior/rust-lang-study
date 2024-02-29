fn main() {
    {
        fn addOne(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(addOne, 10);

        println!("{}", answer);
    }

    {
        enum Status {
            Value(i32),
            Stop,
        }

        let list_of_status: Vec<Status> = (1..20).map(Status::Value).collect();
    }

    {
        use hello_macro::HelloMacro;
        use hello_macro_derive::HelloMacro;
        
        #[derive(HelloMacro)]
        struct Pancakes;
        
        Pancakes::hello_macro();
    }
}

fn A() -> Box<dyn Fn()> {
    fn B() {
        println!("hello");
    }

    Box::new(B)
}

#[macro_export]
macro_rules! my_macro {
    ($($a: expr), *) => {
        println!()
    };
}
