# rust의 고급 기능들
자주 사용하지는 않지만, 가끔 사용할 수도 있는 기능들!

## unsafe rust

rust는 기본적으로 컴파일 타임에 메모리 안정성을 보장하려 노력한다. 하지만 프로그램의 모든 영역을 정적 분석으로 처리할 수 없으며, 운영체제 같은 저수준 프로그래밍이 필요한 경우 러스트가 기본적으로 허용하지 않는 - 안전하지 않은 코드를 작성해야 한다. 따라서 rust도 사용자가 직접 메모리 안정성을 관리하게 하는 `unsafe` 코드를 작성하는 방법을 제공한다.

안전하지 않은 코드는 `unsafe` 키워드를 붙인 블럭에 작성 가능하다.

## unsafe superpower

안전하지 않은 코드에서 수행 가능한 작업들을 묶어 `unsafe superpowers`라고 부른다.

- 원시 포인터(raw pointer) 역참조
- 안전하지 않은 함수 or 메서드 호출
- 안전하지 않은 트레잇 구현
- 가변 정적 변수(`static mut`)에 접근 및 수정
- `union` 필드 접근


`unsafe`는 블록 내부에 대한 대여 검사기(borrow checker)나 안정성 검사를 비활성화하지 않는다. 여전히 검사는 동일하게 이루어지며, 위 언급한 5가지 작업을 추가적으로 진행 가능할 뿐이다. 이 작업들에 대해서는 개발자가 직접 메모리 안전성을 보장해야 한다.

가능하면 `unsafe` 블록을 추상화하고, 외부에는 안전한 API를 제공하도록 하자.

## 원시 포인터
원시 포인터는 C언어의 포인터 개념을 의미한다고 보면 된다. `*` 표시를 붙여 "포인터"임을 명시한다.

- `*const T`: 불변 원시 포인터 (`const type *ptr`)
- `*mut T`: 가변 원시 포인터 (`type *ptr`)

특징은 다음과 같다. 그냥 C 언어의 포인터를 생각하면 쉽다.

- 대여 규칙을 무시할 수 있다. (불변 / 가변 포인터를 동시에 여러 개 ok)
- null이 될 수 있으며, 유효한 메모리를 가리킨다고 보장할 수 없다.
- 사용자가 직접 메모리 정리를 구현해야 한다.

컴파일러에 의해 보장되는 안전성을 포기하지만, 다른 언어(주로 C 계열), 하드웨어와의 통신을 구현할 수 있으며 이론 상 성능을 높일 수 있다. 

```rust
let mut num = 5;

let r1 = &mut num as *mut i32;
let r2 = &num as *const i32;

let addr = 0x012345usize;
let r3 = addr as *const i32;
```

포인터 자체는 `unsafe` 블록이 아니어도 생성될 수 있지만, `unsafe` 블록 내에서만 역참조할 수 있다. 

다만 `unsafe` 블록 외부에 정의되었다고 반드시 포인터가 유효하다고 볼 수는 없다. r1, r2는 이미 존재하는 num을 참조하므로 유효하지만, r3는 존재를 알 수 없는 주소를 가리키므로 유효성을 보장하기 어렵다.

## 안전하지 않은 함수 or 메서드 호출
함수 or 메서드 정의 앞에 `unsafe`가 추가하는 경우, 해당 기능은 `unsafe` 블록 내에서만 사용할 수 있다. 현재 기능이 메모리 안전성을 보장하지 않음을 표현한다.

```rust
let num = 10;

unsafe {
    let ptr = get_unsafe_ptr(&num);
    println!("{}", *ptr);
}

unsafe fn get_unsafe_ptr(target: &mut i32) -> *const i32 {
    let ptr = target as *mut i32;
    *ptr += 10;
    return ptr;
}
```

`get_unsafe_ptr` 메서드는 원시 포인터를 이용하여 값을 변경한 후 원시 포인터를 반환한다. `unsafe` 블록 바깥에서는 실행되지 않는다.

### 안전하지 않은 코드를 감싸는 추상화 만들기
```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = values.len();
    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}
```
위 코드는 동작하지 않는다. 값을 반환하는 과정에 values에 대한 가변 참조가 2번 발생하기 때문이다. 각각의 반환값은 values에 대한 가변 참조자로 동작한다.

rust의 빌림 규칙에 따르면 가변 참조는 한번에 하나만 허용되기 때문에, 위와 같이 한번에 여러 가변 참조가 발생하는 코드를 작성할 수 없다.

따라서 위 코드가 동작하기 위해서는 `unsafe` 코드가 필요하다.

```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = values.len();
    // let ptr = values as *mut [i32] as *mut i32;
    let ptr = values.as_mut_ptr();
    assert!(mid <= len); // mid가 범위 내에 있음을 보장

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```
- values.as_mut_ptr: 원시 가변 포인터를 얻는다. 배열 포인터를 개별 값에 대한 포인터로 다시 캐스팅한다.
- slice::from_raw_parts_mut: raw pointer을 통해 슬라이스를 얻는다.

### extern 함수로 외부 코드 호출
C언어 같은 다른 언어로 작성된 코드와 상호작용할 때 extern 키워드를 사용할 수 있다.

```rust
let v = -3;
unsafe {
    println!("{v}의 절대값 == {}", abs(v));
}

extern  "C" {
    fn abs(input: i32) -> i32;
}
```

외부에 코드를 노출할 때는 `#[no_mangle]` 과 `extern  "ABI"`를 명시한다.

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("called rust code from C");
}
```

mangling은 컴파일러가 컴파일 과정에 함수의 이름을 자신이 이용하기 좋게 여러 정보를 포함하도록 변경하는 것을 의미한다. 다른 언어에서 함수를 호출하려면 정확한 이름이 필요하기 때문에, 이 이름을 변경할 수 없도록 방지한다.

## 가변 정적 변수 접근 & 수정
러스트도 정적 변수(static)를 선언할 수 있다. 정적 변수는 프로그램과 생명주기를 같이 하기 때문에 `'static` 라이프타임을 가지고 있으며, 스코프를 벗어나더라도 `drop`이 실행되지 않는다는 특징이 있다.

상수(const)와 달리 정적 변수의 값은 메모리 상 고정된 주소를 가지고, 여러 위치에서 동일한 데이터에 접근할 수 있으므로 멀티 스레드 환경에서 접근할 때 안전하지 않다. 따라서 러스트에서는 정적 변수가 가변으로 정의된 경우 (`static mut`), `unsafe` 환경에서만 접근할 수 있게 한다.
```rust
static mut COUNTER: u32 = 0;

unsafe {
    println!("{}", COUNTER);
    COUNTER += 1;
}
```
## 안전하지 않은 트레잇
메서드 중 하나 이상이 `unsafe`한 코드를 포함하는 경우 트레잇이 안전하지 않음을 표현하기 위해 `unsafe` 키워드를 앞에 추가한다.

```rust
unsafe trait Foo {
    // 여기에 메소드가 작성됩니다
}

unsafe impl Foo for i32 {
    // 여기에 메소드 구현이 작성됩니다
}
```

## 유니온 필드 접근
주로 C언어와의 호환성을 위한 것. union으로 선언된 타입에 접근할 때 사용한다.

[공식 문서](https://doc.rust-lang.org/reference/items/unions.html)