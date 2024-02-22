# 커맨드라인 프로그램 만들기 실습
별도의 설명 없이 실습 위주로 진행

1. 커맨드 인수 읽기
```rust
use std::env;
fn main() {
    // 이터레이터를 벡터 같은 컬렉션으로 변경
    let args: Vec<String> = env::args().collect();

    println!("{:#?}", args);
}
```
첫번째 인수는 실행한 프로그램의 절대 경로이고, 이후부터 입력한 인수를 얻을 수 있음.

사용자 입력 값만 원한다면 인덱스 1 이상을 살펴봐야 함.

유효하지 않은 유니코드를 포함하는 경우 ```std::env::args_os``` 이용.
```
cargo run -- hello world
[
    "target\\debug\\chapter12.exe",
    "hello",
    "world",
]
```


## 관심사 분리
프로그램 규모가 작을 때는 main.rs에 모두 서술해도 상관 없지만, 복잡도가 높아질수록 관리가 까다롭다.

프로그램 주요 로직은 lib.rs에, 이를 이용하는 로직은 main.rs에 정의한다.

- main.rs
    - 인수 값으로 커맨드 라인 파싱 로직 호출
    - 설정
    - lib.rs의 run 함수 호출
    - run이 에러 반환하면 에러 처리
- lib.rs
    - 처리 로직을 서술


## 사용되는 기능들
```rust
// 슬라이스. 범위 내 값을 가져올 수 없으면 텅 빈 배열.
let additional_args = &args[2..];

// 환경 변수 가져오기
std::env::var(&key)

// 프로그램 인수 가져오기. 첫번째 인자는 실행 파일의 경로
let args: Vec<String> = env::args().collect();

// if문은 expression!
let results = if config.ignore_case {
    search_insensitive(&config.query, &contents)
} else {
    search(&config.query, &contents)
};

// eprintln!으로 표준 에러 출력 가능
eprintln!("{err}");
```
표준 출력, 표준 에러 전달하기
```
// 표준 출력
cargo run to poem.txt UPPER_CASE > out.txt

// 표준 에러
cargo run 2> err.txt
```