# rust 언어 시작

## rustc로 컴파일

main.rs 파일을 만들고, 내용을 채운다.

```rust
fn main() {
  println!("hello, world!");
}
```

rustc 컴파일러를 이용하여 컴파일을 진행한다.  

```rustc main.rs```

컴파일 된 결과물을 실행한다.  

```
- main.exe
- main.pdb
- main.rs
```

``` ./main.exe ```

## cargo 이용
cargo는 빌드 시스템 & 패키지 관리자에 해당. 빌드, 라이브러리 다운로드 등 수행  
( npm 같은 역할 )  

단순한 프로젝트는 rustc만 이용해도 큰 문제가 없겠지만, 여러 종속성을 사용하는 등 규모가 확장되기 시작하면 빌드 & 종속성 관리를 처리해주는 cargo가 필요.

구체적인 명령어는 ```cargo --help```로 얻을 수 있음.

### 프로젝트 생성
``` cargo new project_name ```

cargo로 프로젝트를 만들면 버전 관리 파일인 Cargo.toml이 함께 생성됨
```toml
[package]
name = "chapter01_01"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
- package: 현재 패키지에 대한 정보들
- dependencies: 프로젝트 종속성 목록

### 빌드 & 실행
- 빌드만: ```cargo build```
- 빌드 & 실행: ```cargo run```
- 빌드 없이 체크만: ```cargo check```
- 릴리즈 빌드: ```cargo build --release```

릴리즈 빌드의 경우 target/release에 파일을 생성. 컴파일 시간이 길어지지만, 성능이 좋다.