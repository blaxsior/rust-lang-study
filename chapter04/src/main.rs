fn main() {
    // let x = 10;
    // let y = x;

    // println!("{x} {y}");

    // let s1 = String::from("hello");
    // let mut s2 = s1.clone();

    // println!("{s1} {s2}");
    // borrow_your_string(&s1);
    // change_string(&mut s2);
    // println!("{s1} {s2}");

    let mut mystr = String::from("this is test string");

    let readonly_str1 = &mystr;
    let readonly_str2 = &mystr;

    // println!("{readonly_str1}");
    // println!("{readonly_str2}");

    // let mystr2 = &mut mystr;
    // mystr2.push_str("bb");
    // println!("{mystr}");
    // println!("{readonly_str1}");
    // println!("{readonly_str2}");

    // let mystr3 = &mut mystr;
    // // mystr3.push_str("dd");
    // mystr2.push_str("tt");
    // println!("{mystr}");

    let mystr4 = String::from("hello string");
    let mystr5 = String::from("test");
    let mystr6 = String::from("");

    println!("{}", get_first_word(&mystr4));
    println!("{}", get_first_word(&mystr5));
    println!("{}", get_first_word(&mystr6));
}

fn take_your_string(string: String) {
    println!("my string is {string}");
}

fn borrow_your_string(string: &String) {
    println!("i borrow this string: {string}");
}

fn change_string(mystr: &mut String) {
    mystr.push_str(" added");
}

// fn dangling() -> &String {
//     let s = String::from("dangling");

//     return &s;
// }

fn get_first_word_length(s: &String) -> usize {
    //     let values: Vec<&str> = s.split_whitespace().collect();
    //     return values.first().unwrap();

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_first_word(s: &String) -> &str {
    let values: Vec<&str> = s.split_whitespace().collect();
    return match values.first() {
        Some(word) => word,
        None => s, // 문자열이 비어 있는 경우 그대로 반환
    };
}
