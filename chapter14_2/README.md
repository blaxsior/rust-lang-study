# workspace
서로 연관된 여러 패키지를 관리할 수 있는 기능

## 폴더 구조
```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

## 작업 공간 사용하기
1. 최상위 폴더에 Cargo.toml 파일을 생성한다.
2. ```[workspace]``` 을 명시하여 현재 파일이 workspace을 의미함을 알린다.
3. ```members = []``` 을 추가해서 현재 workspace에 속한 멤버를 명시한다.
```toml
[workspace]

members = [ 
  "add_one",
  "adder",
]

```
cargo를 이용해서 패키지를 만들면 새로운 패키지가 members에 자동으로 추가된다.

workspace을 만들면 실행 파일을 만드는 target 폴더나 의존성을 정리하는 Cargo.lock이 workspace 루트 폴더에 생긴다. 의존성을 workspace 단위로 공유된다. 대신, 의존성 자체는 각 패키지마다 정의해줘야 한다.

```cargo test -p 패키지_이름```으로 별개 패키지를 테스트할 수 있다.

add_one => rand 따로
```toml
[package]
name = "add_one"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

adder => rand 따로
```toml
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]
add_one = { path = "../add_one" }
rand = "0.8.5"
```

서로 다른 로컬 패키지에서 접근할 때는 경로를 표기하여 사용한다.
```toml
add_one = { path = "../add_one" }
```