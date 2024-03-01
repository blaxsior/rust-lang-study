use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread, time::Duration,
};

mod threadpool;

use threadpool::ThreadPool;

fn main() {
    // bind는 포트 바인딩을 의미
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
    let pool = ThreadPool::new(4);

    // 스트림에 대한 이터레이터를 기반으로 무한 처리
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

        // thread::spawn(||{
        //     handle_connection(stream);
        // });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // let mut buf = String::new();
    // stream.read_to_string(&mut buf).unwrap();

    // let req: Vec<String> = buf
    // .lines()
    // .map(|it| it.to_string())
    // .take_while(|it| !it.is_empty())
    // .collect();

    // 버퍼 처리 과정을 간단 & 효율적으로 만들어 주는 라이브러리

    let buf_reader = BufReader::new(&mut stream);
    let req: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();


    let (status_line, filename) = if req[0] == "GET / HTTP/1.1" {
        thread::sleep(Duration::from_secs(3));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}

// 스레드 풀 기반으로 처리
// 무한스레드 => DoS 공격 문제
// 스레드 풀을 통해 동시에 N개의 문제 처리.

// fork/join
// 싱글 스레드 async 이벤트 루프
// 멀티 스레드 async 비동기
