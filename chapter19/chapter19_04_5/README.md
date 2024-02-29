# advanced function & closure

## 함수 포인터
클로저에 대한 포인터는 다음과 같았다.

|트레잇|설명|캡쳐 방식|호환|
|-|-|-|-|
|FnOnce|캡쳐된 값을 소비(소유권 이동)할 수 있는, 한 번만 호출 가능한 클로저|이동(move)|FnMut, Fn|
|FnMut|캡쳐된 값을 변경 가능한 클로저|가변 참조|Fn|
|Fn|캡쳐된 값을 변경하지 않는 클로저|불변 참조|-|

함수 포인터는 `fn` 타입만 존재하며, `FnOnce`, `FnMut` 및 `Fn`을 모두 구현하므로 클로저 자리에 언제나 함수를 전달할 수 있다.

C언어 등 클로저가 없는 언어와 상호작용하는 경우 `fn`만 허용한다.

```rust
fn addOne(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

let answer = do_twice(addOne, 10);

println!("{}", answer);
```

열거형 배리언트의 이름도 일종의 생성자 함수이므로 함수 포인터 자리에 넘길 수 있다.
```rust
enum Status {
    Value(i32),
    Stop,
}

let list_of_status: Vec<Status> = (1..20).map(Status::Value).collect();
```

## 클로저 반환
함수나 클로저를 반환하기 위해서는 `FnOnce`, `FnMut`, `Fn` 같은 트레잇을 통해 반환한다. 이때 트레잇은 구체적 타입이 아니므로 `Box<dyn Fn>` 처럼 트레잇 객체로 반환해야 한다.
```rust
fn A() -> Box<dyn Fn()> {
    fn B() {
        println!("hello");
    }

    Box::new(B)
}
```

# 매크로
(나중에 필요한 경우 자세하게 보자)

[관련 문서](https://doc.rust-kr.org/ch19-06-macros.html)

기능 또는 문법 등을 구현하기 위한 코드. 크게 2개 종류로 구분

1. 선언적(declarative) 매크로: `macro_rules!`, 패턴을 기반으로 코드를 단순 대체
2. 절차적(procedural) 매크로: 입력 코드를 기반으로 코드를 생성
    - `#[derive]` 매크로: 구조체, 열거형 등에 특성을 자동 구현하는 매크로
    - `attribute-like` 매크로: 커스텀 속성을 정의하는 매크로
    - `function-like` 매크로: 함수 호출처럼 보이는 매크로
## 매크로 vs 함수
- 매크로: 메타 프로그래밍(metaprogramming), 프로그램을 데이터로 취급하여 작성 + 수정
    - 보일러 플레이트 제거에 도움
    - 가변 매개변수 가능
    - pre processor에 의해 컴파일 타임 이전에 확장됨
    - 정의 & 호출 전에 스코프로 가져와야 함?
- 함수: 특정 기능을 위한 명령들을 추상화한 것.
    - 매개변수 개수 및 타입 명시 필요
    - 컴파일 타임에 고정적으로 구현되어 있어야 함
    - 어디서나 정의 & 호출 가능

## macro_rules!: 메타 프로그래밍을 위한 선언적 매크로
컴파일 타임에 조건에 맞는 코드를 대체하기 위한 매크로. 패턴을 매칭하여 매칭된 부분을 다른 코드로 대체한다. 복잡한 패턴 매칭을 기반으로 텍스트를 교체하는 규칙.

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```
- `#[macro_export]`: 매크로를 외부 스코프에서 사용할 수 있게 하는 어노테이션
- `macro_rules!`: 매크로 정의를 시작하는 부분
- `vec`: 매크로 정의(이름)
- `( )`: 영역을 감싸기
- `$( $x: expr ), *`: 패턴에 매칭될 매크로 변수 선언
    - `$x: expr`: 러스트 표현식과 매칭되는 변수 부분임을 표시
    - `, *`: 매칭되는 것이 0개 이상임을 의미
```rust
let v: Vec<u32> = vec![1, 2, 3]; // 이 매크로는

let mut temp_vec = Vec::new(); // 대략 아래 코드로 대체된다.
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
```
## 절차적 매크로
코드를 입력받아 작업을 수행한 후, 대응되는 코드를 출력으로 생성.

- 커스텀 파생(custom derive): derive 매크로 관련
- 속성형(attribute-like): 속성처럼 사용
- 함수형(function-like): 함수 호출처럼 보임

내가 공부하는 현재 시점, 절차적 매크로는 별개 크레이트에 정의되어야 한다. (추후 조정될 수 있음.)


### 커스텀 derive 매크로

```
cargo new hello_macro_derive --lib
```

derive에 들어갈 매크로를 정의할 라이브러리를 만든다. 이후 해당 라이브러리의 Cargo.toml에 다음과 같은 요소를 추가한다.

```toml
[lib]
proc-macro = true

[dependencies]
quote = "1.0.35"
syn = "2.0.52"
```
- proc-macro: 현재 라이브러리가 절차적 매크로에 대한 것임을 표현
- syn: 러스트 소스코드를 파싱(TokenStream -> AST)
- quote: 작성한 코드를 러스트 코드(TokenStream)으로 변환

proc-macro = true로 설정하면 매크로 관련 어노테이션을 추가한 경우에만 외부로 노출할 수 있다.

매크로는 메타 데이터를 기반으로 코드를 생성하는 것이 목적이다. 이를 위해 (1) rust의 소스 코드를 읽고 (2) 파싱하여 정보를 얻은 후, (3) 필요한 코드를 생성할 필요가 있다.

이 문서 작성 시점에는 다음과 같은 라이브러리가 각 역할을 수행한다.

1. 소스코드 읽기 => proc_macro: 러스트 코드를 읽고 조작하는 컴파일러 API (기본 제공)
2. 코드 파싱 => syn: TokenStream을 파싱하여 DeriveInput을 생성
3. 코드 생성 + 코드 변환 => quote: 작성한 코드를 TokenStream으로 다시 변환

```rust
use proc_macro::TokenStream; // 코드 수준에서 러스트 코드를 읽고 조작할 수 있게 하는 컴파일러 API

// 코드 파서 기능
use quote::quote; // syn 데이터 구조를 다시 러스트 코드로 변환
use syn; // 러스트 코드를 문자열에서 연산 가능한 데이터 코드로 파싱

// 토큰 스트림 파싱
// 스트림을 파싱하여 원하는 조작을 수행한 후 다시 코드로 변환하여 삽입
#[proc_macro_derive(HelloMacro)] // HelloMacro 트레잇에서 파생되는 매크로임을 표현
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 연산 가능한 데이터 구조로 변환
    let ast = syn::parse(input).unwrap();

    // ast 트리 변환
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // 반환할 코드 정의
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
// stringify!() : 문자열 리터럴로 변환
```
절차적 매크로는 TokenStream을 입력 받고 TokenStream을 반환. 여기서 TokenStream이 rust의 소스 코드에 대응된다.

- `#[proc_macro_derive(HelloMacro)]`: 현재 함수가 `HelloMacro` 트레잇에서 파생되는 매크로임을 표현
- TokenStream: 러스트의 소스 코드에 대응되는 토큰 스트림
- syn::parse: 토큰 스트림을 파싱하여 AST를 생성
- DeriveInput: syn 라이브러리에 의해 생성된 AST. 여러 정보를 포함.
    ```rust
        DeriveInput {
            // --생략--

            ident: Ident {
                ident: "Pancakes",
                span: #0 bytes(95..103)
            },
            data: Struct(
                DataStruct {
                    struct_token: Struct,
                    fields: Unit,
                    semi_token: Some(
                        Semi
                    )
                }
            )
        }
    ```
- quote!: `quote` 라이브러리에 포함된 매크로. rust의 코드를 토큰 스트림으로 변환. `#`을 이용하여 변수를 대치하는 것도 가능함.

위와 같이 매크로를 위한 패키지를 만든 후, 대상이 되는 트레잇인 `HelloMacro` 트레잇을 별개의 라이브러리에 구현한다.

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

이후 두 라이브러리를 스코프로 가져와 사용하면 컴파일 시 매크로에 의해 코드를 생성된다.
```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

Pancakes::hello_macro();
```

### 속성형 매크로
새로운 속성을 생성하여 사용할 수 있는 매크로. `#[proc_macro_attribute]` 어노테이션이 필요하다.
```rust
#[route(GET, "/")]
fn index() {
    // do something
}
```

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    //do_something
}
```

전체적인 구현은 derive와 거의 동일하게 처리하면 된다. 속성형 매크로는 함수의 이름, 변수와 매칭되므로, 이를 이용한 코드를 생성하게 구현한다.

### 함수형 매크로

`#[proc_macro]`를 적용한다. 함수처럼 사용되는 `macro!()` 형식의 매크로를 정의하는데 사용된다. 

선언적 매크로도 `macro!()` 형식의 매크로를 정의하는데 사용될 수 있었다. 차이점을 대략 정리하면 다음과 같다.

선언적 매크로는 전용 구문을 사용해서, 함수형 매크로는 TokenStream을 조작하여 코드를 생성한다는 부분에 차이가 있다. rust의 많은 `macro!()` 형식의 매크로는 선언적 매크로를 따른다.

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
}
```
