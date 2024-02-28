# 고급 타입(advanced types)
- 뉴타입 패턴: 기존 타입을 감싸 새로운 타입으로 다루는 패턴
- 타입 별칭: 타입에 기존과 다른 이름을 부여하는 방법
- `!` 타입: 값을 절대 반환하지 않음을 표현(코틀린의 Nothing과 유사한듯?)
- Sized 트레잇: 컴파일 타임에 타입의 크기를 알 수 있음을 의미하는 트레잇

## 뉴타입 패턴(newtype pattern)
단일 필드의 튜플(`struct NewType(type)`...)로 기존 타입을 감싸 새로운 타입을 만드는 패턴. 값의 의미를 명확하게 표현하고, 캡슐화를 통해 구현의 공개 여부를 조절하기 위한 목적 등으로 사용된다. 

타입 별칭(type alias)과는 달리 실제 타입이 생성되므로, 값을 의도한 위치에서만 사용하게 강제하는 것이 장점이다.

printName은 입력이 실제로 "이름"인 경우에 대해서만 동작해야 한다. 입력 타입을 &str로 두는 대신 String을 FullName으로 감싸 새로운 타입을 만들면 printName에 "이름"만 입력할 수 있게 된다.

```rust
struct FullName(String);

fn printName(name: &FullName) {
    println!("name: {}", name.0);
}

fn do_something(text: String) {
    //do something
}

let name = FullName(String::from("hong gil dong"));

printName(&name); // 가능
do_something(name); // 불가능
```

[effective rust item 7](https://www.lurklurk.org/effective-rust/newtype.html)에서는 뉴타입 패턴을 사용하는 예시를 보여준다. 타입 별칭(type alias)과는 달리 실제 타입이 생성되므로 사용자의 의도를 코드 상에 반영하는데 큰 도움을 준다.

## 타입 별칭(type alias)
타입에 다른 이름을 부여하는 기능. 타입을 부르는 이름만 바뀔 뿐, 타입 자체는 동일하다.

값의 의미를 표현하는데 사용될 수 있지만, 내부적으로 타입 자체는 동일하게 취급되므로 완전히 다른 타입이 되는 뉴타입에 비해 안전하지 않다. 일반적으로 긴 타입 정의를 짧게 줄여 관리하는데 도움이 된다.

```rust
type FullName = String;

let text1 = String::from("hello");
let name: FullName = String::from("hong gil dong");
// 다른 이름으로 표현할 수 있을 뿐, 타입 자체는 동일.


// 긴 타입을 짧은 이름으로 묶어 관리하기 쉬움
type lambda = dyn Fn() + Send + 'static;
let f: Box<lambda> = Box::new(|| println!("hello"));
```

## 부정 타입 `!`: 반환 값이 없음을 표현
`!` 타입은 "값이 없음"을 의미하는 타입으로, 함수가 무한히 실행되거나, 패닉을 발생시키는 용도로 사용하는 등 값을 절대 반환하지 않을 때 반환 타입에 사용된다.

빈 타입(empty type) 또는 부정 타입(never type)으로 불린다.

코틀린의 Nothing과 거의 유사한 개념으로 보면 된다. continue, loop, panic! 등은 `!`을 반환하고, 코틀린에서 Nothing이 모든 타입의 서브 타입처럼 간주된 것처럼 러스트에서는 `!`가 모든 타입으로 강제 변환될 수 있다고 설명한다.

```rust
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
```
## 동적 크기 타입 & Sized 트레잇
러스트는 컴파일 타임에 타입에 할당할 메모리 크기 등을 알고 있어야 하고, 타입이 같다면 동일한 크기의 메모리를 사용해야 한다. 그런데, 경우에 따라 이런 조건을 만족하지 않는, 동적인 크기를 가지는 타입도 존재한다.

동적 크기 타입(dynamically sized type, DST)은 크기가 지정되지 않고, 런타임 시점에만 크기를 알 수 있는 값을 의미한다. 문자열 리터럴을 의미하는 str 같은 경우가 DST에 해당한다.

```rust
let s1 = "hello, world!";
let s2 = "study me!";
```
위 정의된 두 문자열 리터럴은 서로 다른 크기를 가지고 있다. 이때 러스트에서 이 둘을 같은 `str`이라는 타입으로 다루기 위해서는 동일 크기의 메모리를 차지해야 하지만, 실제로는 다른 길이의 메모리를 요구하므로 `str`을 직접 다룰 수 없다.

대신 문자열의 시작, 끝 지점을 표현하는 문자열 슬라이스인 `&str`을 사용하여 문자열 리터럴을 표현한다. 문자열 자체의 길이가 다르더라도 시작, 끝 지점을 표현하는 크기는 동일하므로 타입으로 사용될 수 있다.

이처럼, 러스트는 동적 크기 타입 자체를 표현하는 대신 이를 가리킬 수 있는 메타데이터를 포함한 포인터를 통해 간접적으로 표현한다.

Sized 트레잇은 특정 타입의 크기가 컴파일 타임에 알 수 있음을 표현한다. 컴파일 단계에서 크기가 알려져 있는 경우 자동으로 구현되며, 제네릭 파라미터도 자동으로 Sized 트레잇 바운드를 포함하게 된다.

```rust
fn generic<T: Sized>(t: T) {
    // --생략--
}
```

`?`를 앞에 붙이면 컴파일 타임에 크기가 알려져 있을 수도 있고 아닐 수도 있다는 의미를 가지게 된다. 현재는 Sized 트레잇 앞에만 붙일 수 있다. 이때 컴파일 타임에 크기를 알 수 없다면 포인터로 접근해야 하므로 변수 t는 참조(&T)로 접근해야 한다.

```rust
fn generic<T: ?Sized>(t: &T) {
    // --생략--
}
```

(나중에 새로운 내용을 알게 되면 추가)