use std::{any::{Any, TypeId}, error::Error, fs::File, io::{self, ErrorKind, Read}};

fn main() {
    // let v = vec![1,2,3];
    // v[99];

    // Result 열거형 다루기
    {
        use std::fs::File;
        use std::io::{ErrorKind, Read};

        let filename = String::from("test.txt");

        let file_result = File::open(&filename);

        // let mut file = match file_result {
        //     Ok(f) => f,
        //     Err(err) => match err.kind() {
        //         ErrorKind::NotFound => match File::create(&filename) {
        //             Ok(f) => f,
        //             Err(_) => panic!("cannot create file: {filename}"),
        //         },
        //         other => panic!("cannot open file: {filename}")
        //     }
        // };

        // let mut file = File::open(&filename).unwrap_or_else(|err| {
        //     if err.kind() == ErrorKind::NotFound {
        //         File::create(&filename).expect("cannot create file")
        //     } else {
        //         panic!("cannot open file")
        //     }
        // });

        // let mut buf = String::new();

        // if let Ok(size) = file.read_to_string(&mut buf) {
        //     println!("file content: {buf}");
        // }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = match File::open("username.txt") {
        Ok(f) => f,
        Err(e) => return Err(e) // 외부로 에러를 리턴
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_from_file2() -> Result<String, OurError> {
    // let mut file = File::open("username.txt")?;
    // let mut username = String::new();
    // file.read_to_string(&mut username)?;
    // Ok(username)

    let mut buf = String::new();
    File::open("username.txt")?.read_to_string(&mut buf)?;

    Ok(buf)
}

struct OurError {
    kind: ErrorKind
    // 이외의 여러 에러 관련된 것들
}

impl From<io::Error> for OurError {
    fn from(value: io::Error) -> Self {
        OurError {
            kind: value.kind(),
        }
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}