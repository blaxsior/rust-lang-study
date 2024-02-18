# 구조체(struct)
rust 클래스, 상속 개념이 없다. 구조체 통해 객체를 구현할 수 있다.

## 구조체의 정의
### 일반 구조체
이름 있는 필드를 가지는 기본적인 구조체
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new(active: bool, username: String, email: String) -> Self {
        Self {
            active,
            username,
            email,
            sign_in_count: 0,
        }
    }
}

impl User {
    fn setActive(&mut self, active: bool) {
        self.active = active;
    }

    fn inc_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }
}
```
- 필드: 구조체 본문에 작성한다.
- 메서드: impl 블록을 따로 만들어 작성한다. impl 블록은 여러개 둘 수 있다.  
제네릭 타입과 trait을 다룰 때 사용될 수 있다.

메서드에서 자신의 필드에 접근하려면 self 키워드를 이용한다. 메서드가 self를 받을 때 함수와 마찬가지로 구조체 자체에 대한 소유권이 이동하므로, ```&```를 붙여 참조자를 제공해야 한다. 변경하는 부분이 있다면 ```mut```가 필요하다.

```impl``` 블록 내부에서는 구조체 자신을 의미하는 파라미터인 ```self```와 구조체 자신의 타입을 의미하는 ```Self```을 이용할 수 있다.

#### Associated function
자기 자신을 인스턴스로 받지 않는(self 파라미터가 없는) 함수. 일반적인 언어의 정적 메서드에 대응된다.
```rust
impl User {
    fn new(active: bool, username: String, email: String) -> Self {
        Self {
            active,
            username,
            email,
            sign_in_count: 0,
        }
    }
}
```

### 튜플 구조체(Tuple Structs)
named tuple같은 역할을 하는 구조체. 필드에 이름이 없다.
```rust
let rgb = RGB(0,0,0);
let pos2d = Vec2D(10, 3);

struct RGB(i32, i32, i32);
struct Vec3D(i32, i32, i32);
struct Vec2D(i32, i32);
```
각 필드와 연결된 이름이 없으며, 인덱스로 접근할 수 있다.

두 구조체가 가진 필드의 개수 및 타입이 같더라도 다른 타입이므로 호환은 안된다.

### 유사 유닛 구조체(Unit-Like Structs)
rust에서 Unit은 어떤 값도 없는 튜플로, 빈 값을 의미하며 ```()```으로 표현된다. 유사 유닛 구조체는 Unit과 비슷하게 동작하는 구조체로, 어떤 필드도 가지지 않는다.
```rust
struct AnyAnimal;

let animal = AnyAnimal;
```
trait을 이용, 일종의 인터페이스 기능을 구현할 때 유용한 것으로 보인다.

