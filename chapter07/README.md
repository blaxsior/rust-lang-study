# 패키지, 크레이트, 모듈로 프로젝트 관리하기
나중에 잘 이해하면, 이해한 내용을 바탕으로 다시 작성해보자...

프로젝트 규모가 커지면 하나의 파일로 관리하기 어렵다. 따라서 연관된 코드를 잘 묶고 분리하는 구조화를 통해 코드를 잘 관리해야 한다.

rust 역시 구조화를 위한 여러 기능을 제공하고 있다.

- 패키지: 크레이트 빌드, 테스트, 공유하는데 사용되는 cargo의 기능
- 크레이트: 라이브러리, 실행 가능한 모듈로 구성된 트리 구조
- 모듈 & use: 구조, 스코프를 제어 & 조직의 세부 경로를 감추기
- 경로: 구조체, 함수, 모듈 등의 이름을 지정

# 패키지& 크레이트
패키지는 현재 프로젝트를, 크레이트는 프로젝트에 포함된 단위를 의미

1. 크레이트:  
컴파일 한 차례에 고려하는 가장 작은 코드 단위 =  ```rs``` 확장자의 소스 파일
    - 바이너리 크레이트: 실행 파일로 컴파일 가능한 프로그램. main 함수를 포함
    - 라이브러리 크레이트: main함수가 없는, 공용될 의도로 정의하는 크레이트. "라이브러리"로 해석하면 됨.
    - 크레이트 루트: 러스트 컴파일러가 컴파일을 시작하는 소스 파일. 루트 모듈을 구성
2. 패키지:  
크레이트의 묶음. 하나의 프로젝트는 하나 이상의 패키지로 구성된다.

Cargo.toml 파일로 정의되며, 프로젝트의 메타데이터 / 의존성을 포함한다.

하나의 패키지는 최대 1개의 라이브러리 크레이트와 여러 개의 바이너리 크레이트를 포함할 수 있다. 종류와 관계 없이 크레이트 하나는 반드시 포함한다.

Cargo.toml => 패키지를 만들어주는 파일

- ```src/main.rs```나 ```src/lib.rs``` 같은 파일을 크레이트 루트 파일이라고 부르는데, 관례적으로 cargo는 해당 파일이 패키지 내에 존재하면 패키지 명과 동일한 바이너리 / 라이브러리 크레이트가 있다고 판단하므로, 굳이 Cargo.toml 내에 명시하지 않는다.
- 크레이트 루트 파일은 빌드할 때 컴파일러인 ```rustc```에 전달된다.
- 여러 바이너리 크레이트를 포함하려면 ```src/bin``` 디렉터리에 파일을 배치한다.

# 모듈
코드를 조직화하는 단위. 파일 또는 모듈 정의를 기준으로 모듈 트리를 구성할 수 있다.

파일 또는 블록(``{}``)을 사용하여 정의할 수 있으며, 디렉토리처럼 트리 구조로 모듈을 표현한다.

내가 이해하기로는 크게 2가지 방식으로 모듈을 나타낼 수 있다.
1. 파일 자체를 모듈로 사용한다.
2. ```mod``` 키워드를 이용하여 정의한다.

모듈을 세부 폴더에 선언하는 경우, 과거 파이썬에서 모듈을 구성할 때 ```__init__.py``` 파일을 만들었던 것처럼 모듈 이름 폴더 내에 ```mod.rs``` 파일을 추가하거나, 모듈 이름으로 rs 파일을 만들면 된다.

요약하면, 모듈은 3가지 방법으로 구현할 수 있다.
1. mod 모듈 {세부 사항}
2. src/모듈.rs (최근 스타일)
3. src/모듈/mod.rs (예전 스타일)

파일 구조로 보면 다음과 같다.
1. 동일 이름의 파일 이용
```
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs // 동일 이름의 파일 이용
    └── main.rs
```
2. mod.rs 이용
```
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   ├── mod.rs // mod.rs을 이용
    │   └── vegetables.rs
    └── main.rs
```
파일 안의 내용
```
// mod.rs 또는 garden.rs 안에 정의하는 내용
pub mod vegetables;
```

하나의 파일 내에서 여러 모듈을 선언하면, 다음과 같이 모듈 내에 모듈이 중첩되어 표현된다.
```rust
// in src/lib.js
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
위에서 설명한 mod.rs 또는 garden.rs의 경우 서로 다른 파일로 모듈의 정의가 분리되어 있다. C 언어에서 ```#include``` 을 통해 여러 파일의 내용을 로드 하듯, 여러 파일에 분산된 모듈을 로딩하여 모듈 트리를 만들기 위해 mod.rs / garden.rs 파일에서 세부 모듈을 로드 할 필요가 있다.

모듈과 관련된 키워드는 다음과 같다.
- ```mod``` 키워드: 모듈을 정의하거나, 기존 존재하는 모듈을 가져옴
- ```use``` 키워드: 다른 모듈에 정의된 아이템에 대한 단축 경로를 제공
- ```pub``` 키워드: 모듈 내 아이템을 외부로 공개

## module cheat sheet
- 크레이트 루트: 크레이트를 컴파일할 때 컴파일러는 먼저 크레이트 루트 파일을 본다
    - 바이너리 크레이트: src/main.rs
    - 라이브러리 크레이트: src/lib.rs
- 모듈 선언: 크레이트 루트 파일에는 새로운 모듈을 선언할 수 있다.  
```mod garden;```이라는 코드로 ‘garden’ 모듈을 선언하면, 컴파일러는 아래의 장소에서 이 모듈의 코드가 있는지 살펴본다.
  - mod garden { ~ } 
  - src/garden.rs
  - src/garden/mod.rs
- 서브모듈 선언: 크레이트 루트가 아닌 다른 파일에서는 서브모듈 (submodule) 을 선언할 수 있다. src/garden.rs 안에 mod vegetables;를 선언하면, 컴파일러는 아래의 장소들에서 이 서브모듈의 코드가 있는지 살펴본다.
  - mod vegetables { ~ } 
  - src/garden/vegetables.rs
  - src/garden/vegetables/mod.rs
- 모듈 내 코드로의 경로: 모듈을 크레이트의 일부 구성하면, pub로 공개된 아이템은 해당 코드의 경로를 사용하여 크레이트 내에서 참조할 수 있다.  
예를 들면, garden vegetables 모듈 안에 있는 Asparagus 타입은 crate::garden::vegetables::Asparagus로 찾아 쓸 수 있다.
- 비공개 vs 공개: 기본적으로 private. 공개하고 싶으면 pub 키워드를 붙이자
- use 키워드:  긴 경로의 반복을 줄이기 위한 어떤 아이템으로의 단축경로를 만든다.  
crate::garden::vegetables::Asparagus를 참조할 수 있는 모든 스코프에서 use crate::garden::vegetables::Asparagus;로 단축경로를 만들 수 있으며, 그 이후부터는 스코프에서 이 타입을 사용하려면 Asparagus만 작성하면 된다.

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
## 크레이트 루트
러스트 컴파일러가 컴파일을 시작하는 소스파일로, ```crate``` 루트 모듈을 구성한다.

모듈 트리(module tree) 최상위에 있는 암묵적인 ```crate``` 이름의 모듈로 컴파일된다.
- 바이너리 크레이트: ```src/main.rs```
- 라이브러리 크레이트: ```src/lib.rs```

패키지에서 크레이트 루트 파일은 별도의 이름이 없더라도 패키지 명과 동일한 크레이트가 존재한다고 판단하여 따로 추가하지 않는다.

```rust
// in src/lib.js
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
크레이트 루트에 해당하는 ```src/lib.js``` 파일은 암묵적으로 ```crate``` 라는 이름을 가진 루트 모듈이 된다.
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## 모듈 트리 내 아이템 참조
모듈을 찾기 위한 경로는 2가지가 존재한다.
1. 절대 경로(absolute path): 크레이트 루트에서 시작되는 전체 경로
    - 현재 크레이트: ```crate```에서 시작
    - 외부 크레이트: 크레이트 이름에서 시작
2. 상대 경로: 현재 모듈을 중심으로 ```self```, ```super``` 또는 모듈 내 식별자를 사용하여 접근

러스트의 모듈은 기본적으로 내부의 모든 세부 구현을 숨긴다. 공개하고 싶다면 ```pub``` 키워드를 이용하여 의도적으로 공개해야 한다.

- 부모 -> 자식: 캡슐화에 의해 ```pub```로 공개된 아이템만 볼 수 있다.
- 자식 -> 부모: ```pub```로 공개되지 않더라도 부모 요소를 볼 수 있다.

공식 문서에서는 ```캡슐화```에 의해 부모는 하위 모듈의 공개 아이템만 볼 수 있지만, 하위 모듈은 자신이 정의된 컨텍스트인 부모 모듈을 볼 수 있다고 표현한다.

부모 모듈은 ```super``` 키워드로 접근한다.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {
        }
    }

    fn front_house_func() {}

    pub mod serving {
        pub fn take_order() {
        }

        fn something1() {
            super::front_house_func(); // 부모 요소는 pub 아니어도 접근 가능
        }

        fn something2() {
            super::hosting::add_to_waitlist(); // 부모가 가진 모듈에 접근.
        }
    }
}
```

## use 키워드
C++의 using namespace ~ 와 비슷한 기능.  
반복적으로 사용되는 모듈과 모듈 내 아이템을 이용하기 위해 매번 긴 경로를 명시해야 하는 것은 참 불편하다. use 키워드는 C++의 using과 유사하게 긴 모듈의 이름을 단축하는 방법을 제공한다. 

단축된 이름은 스코프(중괄호로 감싼 범위) 내 어디서든 사용 가능하다. C++에서 클래스 내에 using namespace ~ 을 정의하면 클래스라는 범위 내 어디서든 사용할 수 있는 것과 같다.

```rust
use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    crate::front_of_house::serving::take_order();
    add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
}
```
서로 다른 스코프는 use 구문이 공유되지 않는다.
```rust
mod customer {
    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_somegthing() {
        add_to_waitlist(); 
    }
}
```
```customer``` 모듈 스코프는 크레이트 루트인 ```crate```와 다른 스코프이다. 따라서 super을 통해 ```crate``` 수준에서 사용되는 add_to_waitlist을 가져오거나, 아니면 ```customer``` 스코프 내에 ```use``` 키워드를 별도로 사용해서 단축된 이름을 사용해야 한다.
```rust
mod customer {
    pub fn eat_somegthing() {
        super::eat_at_restaurant();
        super::add_to_waitlist();
    }
}
```
## use 키워드 경로 작성 규칙
1. 함수의 경우, 다른 스코프 출신이라는 것을 명시하기 위해 모듈까지만 단축
2. 구조체나 열거형의 경우 전체 경로에 대해 단축

```rust
// 모듈 로드
mod my_function; // 함수는 모듈까지만
mod some;
use some::SomeStruct; // 열거형이나 구조체는 전체 경로

fn some_function() {
    my_function::do_something();
    let mystruct = SomeStruct;
}
```

## as 키워드
use로 그대로 가져오는 대신, 다른 이름으로 가져오기 위한 키워드.
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --생략--
}

fn function2() -> IoResult<()> {
    // --생략--
}
```
## re-export (pub use)
use는 해당 스코프 내에서만 적용된다. 외부에서도 단축 이름으로 접근할 수 있게 하려면 ```pub use```로 적는다.
```rust
mod customer {
    //as 키워드로 단축 이름 변경 + pub로 외부 공개
    pub use crate::front_of_house::hosting::add_to_waitlist as add_wait;
    pub fn eat_somegthing() {
        super::eat_at_restaurant();
        add_wait();
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::serving::take_order();
    // customer에서 공개 + 재정의된 이름으로 함수에 접근
    customer::add_wait();
    // front_of_house::hosting::add_to_waitlist();
}
```

## 외부 패키지
외부 패키지를 이용하기 위해서는 Cargo.toml 파일의 의존성에 등록하고, use 키워드를 사용하여 현재 스코프로 가져와야 한다.
```rust
mod RandomThings {
    use std::io::stdin;
    use rand::Rng;

    pub fn do_something() {
        let random_number = rand::thread_rng().gen_range(1..=100);

        loop {
            let mut guess = String::new();
            stdin()
                .read_line(&mut guess)
                .expect("입력을 받아오는데 실패했습니다.");
            // 여러 코드들...
        }
    }
}
```
std 표준 라이브러리는 러스트 언어에 포함되어 있어 Cargo.toml에 추가하지는 않지만, 외부 크레이트로 간주된다. guessing_game에서 ```use std::io::stdin```을 입력했던 이유는 std 패키지가 외부 패키지로 취급되기 때문이다.

## 중첩 경로 사용
중괄호 (```{}```)를 이용하여 중첩된 경로를 한 줄로 표현할 수 있다.
```rust
// 나열된 경로. 중첩된 부분 존재
use std::cmp::ordering;
use std::io;

// 중첩된 부분을 합치기.
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;

// 경로가 전부 겹치면, self로 표현
use std::io::{self, Write};
```

## 모두 가져오기
익숙한 ```*``` 기호를 이용한다. 이렇게 사용하면 나중에 정말 불편하기도 하고, 일반적으로 라이브러리에 정의된 모든 아이템을 두루 사용하지는 않기 때문에 가능하면 각각 명시해서 가져오자.
```rust
use std::collections::*;
```