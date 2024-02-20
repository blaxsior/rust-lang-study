# 에러 처리
프로그램은 언제나 에러가 발생할 가능성을 가지고 있다. rust는 가능한 대부분의 에러를 컴파일 타임에 감지하고 다룸으로써, 보다 안전한 프로그램을 배포할 수 있도록 노력한다.

러스트의 에러는 크게 2가지로 나뉜다.
1. 복구 가능한(recoverable) 에러: 사용자와의 상호작용으로 해결 가능한 경우. 파일 찾기
2. 복구 불가능한(unrecoverable) 에러: 버그 증상이 나타나는 경우. out of index

rust는 ```try - catch```을 기반으로 한 예외(exception) 처리 기능이 없다. 예외 처리는 컴파일 타임에 강제하기 어렵고, 성능에 좋지 않기 때문이다. (자바 같은 경우 예외 처리를 강제하는 Checked Exception이 존재하나, 사용하기 불편하여 자주 사용하지 않는다.)

try-catch 기반 에러 처리 대신 Result&lt;T, E&gt;을 이용하여 컴파일 타임에 에러 처리를 강제하며, 가능한 한 에러를 함수 수준에서 명시하며 개발자가 함수의 실패 가능성을 알 수 있게 한다. 이를 통해 보다 안전한, 버그가 적은 코드를 작성할 수 있게 된다.

- 복구 가능한 에러: Result&lt;T, E&gt;
- 복구 불가능한 에러: panic! 매크로

# panic! + 복구 불가능한 에러
프로그램 동작 중 복구할 수 없는, 버그에 가까운 에러가 발생하는 경우 패닉이 발생한다.

패닉이 발생하면 프로그램은 스택을 되감으면서(unwinding) 다른 언어들처럼 문제가 되는 부분에 대한 내용을 출력한다. 이런 기본 동작 대신 바로 프로그램이 종료되게 하려면 Cargo.toml 내부의 설정을 변경한다. (development의 경우 profile)
```toml
[profile.release]
panic = 'abort'
```
## backtrace
어떤 지점에 도달하기까지 호출한 모든 함수 목록. 호출 스택을 되감으며 함수 목록을 출력한다.
```
thread 'main' panicked at src\main.rs:3:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library\std\src\panicking.rs:645 
   1: core::panicking::panic_fmt
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library\core\src\panicking.rs:72 
   2: core::panicking::panic_bounds_check
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library\core\src\panicking.rs:208
   3: core::slice::index::impl$2::index<i32>
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce\library\core\src\slice\index.rs:255
   4: alloc::vec::impl$12::index<i32,usize,alloc::alloc::Global>
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce\library\alloc\src\vec\mod.rs:2770
   5: chapter09::main
             at .\src\main.rs:3
   6: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce\library\core\src\ops\function.rs:250
   7: core::hint::black_box
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce\library\core\src\hint.rs:286
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
# Result + 복구 가능한 에러
rust에는 앞서 언급했듯이 try - catch 구문이 없다. 대신 연산의 함수의 성공 / 실패 여부를 표현할 수 있는 ```Result<T, E>``` 열거형을 제공하여 컴파일 타임에 발생 가능한 에러의 처리를 강제한다.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- Ok: 성공한 경우 반환될 값의 타입인 T를 제공
- Err: 실패한 경우 에러 타입인 E를 제공

Result는 열거형이므로, 컴파일 타임에 가능한 경우의 수인 Ok와 Err를 모두 다루도록 강제된다. 따라서 컴파일 타임에 강제되지 않는 try - catch에 비해 더 안전하다.

더 이상 프로그램을 진행하지 못하는 상황이라면 panic!을 통해 의도적으로 패닉을 발생시킬 수 있다.
## 에러 다루기
```rust
use std::fs::File;
use std::io::{ErrorKind, Read};

let filename = String::from("test.txt");

let file_result = File::open(&filename);

let mut file = match file_result {
    Ok(f) => f,
    Err(err) => match err.kind() { // 에러의 종류를 알 수 있음
        ErrorKind::NotFound => match File::create(&filename) {
            Ok(f) => f,
            Err(_) => panic!("cannot create file: {filename}"),
        },
        other => panic!("cannot open file: {filename}")
    }
};

let mut buf = String::new();

if let Ok(size) = file.read_to_string(&mut buf) {
    println!("file content: {buf}");
}
```
match는 Result 열거형이 가진 다른 메서드를 이용하여 더 간결하게 표현할 수 있다.
```rust
let mut file = File::open(&filename).unwrap_or_else(|err| {
    if err.kind() == ErrorKind::NotFound {
        File::create(&filename).expect("cannot create file")
    } else {
        panic!("cannot open file")
    }
});
```
- expect(err_message): Ok면 T 반환, 아니면 err_message의 패닉 발생
- unwrap(): Ok면 T 반환, 아니면 panic 발생
- unwrap_or(default_value): Ok면 T, 아니면 default_value 반환
- unwrap_or_else(lambda): Ok면 T, 아니면 lambda를 실행한 결과 반환
## 에러 전파하기
실패할 수 있는 무언가를 함수 내부에서 처리하는 대신, 함수를 호출하는 코드에서 다루게 만들고 싶을 수도 있다. 이것을 에러 전파(propagating)이라고 부른다.
```rust
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
```
1. File::open으로 파일 열기를 시도한다.
2. match로 성공했는지 검사한다. 실패한 경우 return을 통해 에러를 반환한다.
3. 버퍼를 만들고, 버퍼에 파일의 내용을 읽어 온다.
4. 잘 읽혔다면 username을, 아니면 에러를 반환한다.

## 에러 전파 숏컷 ```?```
함수는 자신을 호츨하는 코드가 어떤 의도로 자신을 호출하는지 알 수 없으므로, 성공 / 실패 여부를 자신을 호출한 측에 넘겨 의도에 맞게 처리하도록 만들고 싶을 수 있다. 

rust는 이런 경우를 위해 에러를 전파하는 숏컷인 ```?``` 연산자를 제공한다.

```?``` 연산자는 연산의 성공 여부를 다룬다. ```?``` 와 호환될 수 있는 타입은 3가지다.
1. ```Result<T, E>```: 동작의 성공 여부를 다룬다. => Ok(T), Err(E)
2. ```Option<T>```: 아이템의 존재 여부를 다룬다. => Some(T), None
3. ```FromResidual<T>```: 변환 가능 여부? 를 다루는 것으로 보이나, 아직 이해 X

? 연산자와 관련된 영상: https://www.youtube.com/watch?v=NSqN2r0h8DE

```rust
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```
```?``` 연산자는 에러를 얻으면 ```from``` 함수를 호출, 얻은 에러를 현재 함수의 반환형인 ```Result<T,E>```의 에러 타입 ```E```로 변환하여 반환한다. match 기반의 장황한 처리 로직을 제거하고, 코드를 간단하게 작성하기 위해 만들었다고 생각할 수 있겠다.

```rust
fn read_username_from_file2() -> Result<String, OurError> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

struct OurError {
    kind: ErrorKind,
    // 이외의 여러 에러 관련된 것들
}
// io::Error 타입으로부터 OurError을 생성한다. 
impl From<io::Error> for OurError {
    fn from(value: io::Error) -> Self {
        OurError {
            kind: value.kind(),
        }
    }
}

// Option을 이용하는 경우
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```
타입 변환은 ```OurError```의 트레잇인 ```From<io::Error>```에서 다룬다. ? 연산자는 ```OurError```에 정의된 from 메서드를 호출하여 에러를 OurError으로 변환한다. 에러의 변환 과정이 함수와 분리되므로, 코드가 확실히 간결해진다.

```?``` 연산자는 반환 값과 반환 타입이 서로 **호환되어야** 사용 가능하다. 당연하지만, ```OurError```이 ```From<io::Error>``` 트레잇을 구현하지 않았다면 ```?```가 동작하지 않았을 것이다.
***
관례적으로 프로그램이 성공적으로 종료되면 0, 아니면 다른 값을 반환하도록 코드를 작성한다. rust에서도 이런 동작이 가능하다.
```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```
메인 함수의 반환형이 ```Result<(), E>``` 일때, Ok(())을 반환하면 0, Err을 반환하면 0이 아닌 값으로 종료된다. 이 동작을 통해 C 같은 언어와 호환될 수 있다.

# 언제 panic!을 사용해야 하는가
[공식문서](https://doc.rust-kr.org/ch09-03-to-panic-or-not-to-panic.html)를 보면서 한번 고민해보는 것이 더 좋겠다. 

에러 처리에 정답은 없지만 쉽게 예상하고 대응할 수 있는 경우는 Result를, 고장으로 인해 진행이 불가능하거나, 보안적 문제가 발생할 수 있다면 panic!을 이용하는 것이 좋다.

panic!을 사용하면 코드를 더 이상 복구할 수 없으므로 주의하자.

## 가이드라인
아래는 공식 문서의 권장 사항일 뿐 프로그래머의 구현에 달려있다.
|상황 |처리 |
|-|-| 
|함수| 호출하는 측이 복구 여부를 결정하도록 Result를 반환|
|프로토타입|어떻게 다뤄야 할지 결정되지 않았다면 expect나 unwrap을 이용하여 패닉이 발생하게 두고, 나중에 처리 방법이 결정되었을 때 수정|
|테스트| 메서드 호출이 실패했을 때 전체 테스트가 실패할 수 있도록 panic!을 통해 실패를 표시|
|예제|개념을 명확하게 묘사할 수 있도록 panic!과 Result를 적절히 사용|
|컴파일러보다 많은 정보를 가짐|절대 에러가 발생하지 않음을 확신할 수 있다면 반드시 에러처리를 할 필요는 X. 대신 에러가 발생하지 않는 이유를 주석을 통해 구체적으로 명시|

## 의도를 전달하기 위한 panic!
panic!은 내가 **가정**한 규칙을 외부 사용자들이 지키도록 강제하는 **계약**으로 사용할 수 있으며, 이를 통해 내 코드가 의도에 맞게 동작함을 **보장**할 수 있다.

프로그래머는 자신이 작성한 코드가 특정 조건 하에서 동작할 것을 기대한다. 예를 들어 1 ~ 100 사이의 값을 입력 받아 내부 값의 일치 여부를 검사하는 로직을 작성했다고 생각해보자. 

외부에서 해당 범위를 넘어가는 숫자를 입력하는 것은 코드를 작성한 개발자 입장에서 버그에 가깝다. 내부 로직으로 이 상황을 처리할 수도 있겠지만, 외부 사용자에게 1 ~ 100 사이의 값만을 입력하도록 강제하기 위해 범위 바깥의 값이 들어오면 의도적으로 panic!을 발생시킬 수 있다.

가정·보장·계약이 깨지는 나쁜 상태(bad state)인 경우 panic!을 이용한다.