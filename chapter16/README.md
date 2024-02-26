# 동시성(Concurrency)
보통 Concurreny와 Parallelism을 구분하는데, 러스트 공식 문서에서는 한 챕터에서 동시에 다룬다. 따라서 이 챕터에서 설명하는 동시성은 이 둘을 모두 의미할 수 있다.

- 동시성 프로그래밍(concurrent programming): 시분할을 통해 스레드를 번갈아가면서 실행, 동시에 실행되는 것 처럼 프로그래밍
- 병렬 프로그래밍(parallel programming): 여러 코어에서 스레드를 각각 처리

# 스레드
- 스레드: 프로세스 내에서 실행되는 흐름의 단위

프로그램의 연산을 스레드 단위로 쪼개 동시에 일을 수행하면 성능을 높일 수 있지만, 스레드 간 실행 순서의 보장이 없으므로 아래 문제가 발생 가능

- race condition: 스레드 실행 순서에 따라 결과가 일관성 없게 달라지는 현상
- dead lock: 두 스레드가 서로의 작업이 종료되기만 기다리면 무한히 대기하는 현상

이런 현상으로 인해 문제가 발생하더라도 이를 재현하고 수정하기 매우 어렵다. 따라서 멀티 스레드 환경에서는 싱글 스레드보다 데이터 일관성을 지키기 위해 주의해야 한다.

러스트의 스레드는 운영체제의 스레드에 기반하므로, 스레드의 동작은 사용하는 운영체제 및 스레드 스케줄링 방식에 따라 달라질 수 있다.

## 사용법
다른 언어의 사용법과 비슷함.
- spawn: 스레드에 대한 핸들 생성
- handle.join: 스레드가 종료될 때까지 대기

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("spawned thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("main thread: {i}");
}
handle.join().unwrap();
```

thread::spawn을 통해 스레드를 생성한다. 이후 join을 통해 생성된 스레드가 종료되기를 대기하고 있다.

```rust
  use std::thread;

  let v = vec![1,2,3];
  
  let handle = thread::spawn(move || {
      println!("vector: {:#?}", v);
  });

  handle.join().unwrap();
```
스레드에 전달되는 클로저는 외부 변수로부터 소유권을 받는다. 스레드 특성상 join이 없다면 실행 순서가 보장되지 않는다. 만약 메인 스레드가 먼저 종료되거나, 의도적으로 v를 제거한다면, 생성된 스레드는 해제된 메모리에 접근하게 된다.

이런 문제를 해결하기 위해 스레드의 클로저는 외부 변수의 소유권을 받는다. `move` 키워드 필수.

# 메시지 패싱: 스레드 간 데이터 전송
rust는 안전한 동시성 보장을 위해 스레드 간 메시지를 주고 받으며 통신하는 메시지 패싱 방식을 이용한다. 메모리 자체를 공유하는 대신, 통신을 통해 데이터를 공유하므로 좀 더 안전하다.

메시지 교환을 위해 채널(channel) 구현체를 제공한다.

- 채널(channel): 스레드 상에서 데이터를 안전하게 전송하기 위한 개념
- 송신자(transmitter): 메시지를 전송하는 측
- 수신자(receiver): 메시지를 수신하는 측
- 닫힘(closed): 송신자 또는 수신자 중 하나가 drop 된 경우

```rust
{
    // multi producer single consumer
    use std::sync::mpsc; 
    use std::thread;
    use std::time::Duration;


    let (tx, rx) = mpsc::channel();

    thread::spawn( move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv: 무기한 대기
    // recv_timeout: 일정 시간만 대기
    // try_recv: 대기 안함

    if let Ok(data) = rx.try_recv() {
        println!("get immediate data: {}", data);
    }
    // if let Ok(data) = rx.recv_timeout(Duration::from_secs(2)) {
    //     println!("data: {}", data);
    // }
}
```
mpsc는 Multi Provider Single Consumer의 약자로, 하나의 스레드가 여러 송신 단말과 하나의 수신 단말을 가질 수 있음을 의미한다.

채널을 통해 데이터를 전송할 때, **소유권도 함께 이동**한다. 송신 측이 데이터를 보낸 후, 수신 측에서 데이터를 수정하거나 제거할 수 있으므로 이를 송신 측에서 접근하는 것은 안전하지 않다. 따라서 소유권을 함께 넘겨, 가능한 문제를 방지한다.

메서드를 정리하면 다음과 같다.

- mpsc::channel(): 스레드 사이 데이터 공유를 위한 채널을 생성
- Sender::send(data): 송신자 측에서 데이터를 보낸다.
- Receiver
    - recv(): 데이터가 도착할 때까지 대기한다.
    - recv_timeout(t): 데이터를 위해 일정 시간 대기한다.
    - try_recv(): 대기 없이, 현재 데이터가 왔는지 체크한다.
## 채널로 여러 값 보내기
```rust
use std::sync::mpsc; 
use std::thread;
use std::time::Duration;

let (tx, rx) = mpsc::channel::<String>();

thread::spawn(move|| {
    let vals = vec![
        String::from("hello"),
        String::from("world"),
        String::from("from"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    // 종료 후 2초 대기
    thread::sleep(Duration::from_secs(2));
});
// 2초 대기 코드로 인해 2초 더 이어짐.
for rcv in rx {
    println!("received: {rcv}");
}
```
채널로 여러 값을 보낼 수 있다. 이때 receiver는 Iterator로 접근 가능하다. 내부적으로 `recv()` 메서드를 이용하여 대기하므로, 채널이 닫히지(closed) 않는 이상 계속 메시지를 대기할 수 있다.
```rust
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.rx.recv().ok()
    }
}
```
송신 측을 클론(clone)하면 여러 스레드에서 메시지를 보낼 수도 있다. 수신 측의 경우 clone 메서드가 존재하지 않으므로 여러 곳에 보내는 시도 자체가 불가능하다. 
```rust
use std::sync::mpsc; 
use std::thread;
use std::time::Duration;

let (tx, rx) = mpsc::channel::<String>();
let tx1 = tx.clone();
thread::spawn(move|| {
    let vals = vec![
        String::from("hello"),
        String::from("world"),
        String::from("from"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    thread::sleep(Duration::from_secs(2));
});

thread::spawn(move|| {
    let vals = vec![
        String::from("this"),
        String::from("is"),
        String::from("th1"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    thread::sleep(Duration::from_secs(2));
});

for rcv in rx {
    println!("received: {rcv}");
}
```

# 공유 메모리 동시성
- 채널: 값이 송신되면 더 이상 사용할 수 없으므로, 단일 소유권과 유사
- 공유 메모리 동시성: 여러 스레드가 동시에 동일한 메모리에 접근 가능, 복수 소유권과 유사

공유 메모리를 이용하는 경우 소유권 & 소유자 관리가 까다롭다. mutex 등을 통해 처리한다.

## Mutex&lt;T&gt;
상호 배제(mutual exclusion)의 줄임말로, 한번에 하나의 스레드만 데이터에 접근할 수 있도록 허용한다. 

단일 자원에 대한 레이스 컨디션 문제를 방지하는 방법으로, 데이터에 락(lock)을 걸어 한 순간에 한 프로세스만 데이터에 접근할 수 있게 보장한다. 

화장실로 생각해보자. 변기는 한 번에 한 사람만 사용할 수 있어야 하며, 일을 보고 있을 때 다른 사람이 들어오지 못하도록 문을 잠근다. 여기서 "변기"라는 자원을 홀로 사용하기 위한 "잠금"이 뮤텍스에 해당한다. 

2가지 규칙을 기억해야 한다.
1. 데이터 사용 전에는 반드시 lock 요청이 필요하다.
2. 데이터 사용이 끝났다면, unlock을 통해 다른 프로세스가 접근할 수 있게 풀어야 한다.

### Critical Section
- 크리티컬 섹션: 레이스 컨디션이 발생할 수 있는 영역
- 레이스 컨디션: 여러 프로세스가 공유 자원에 동시에 접근하려고 경쟁하는 상태

멀티 스레드 환경에서는 스레드의 실행 순서를 보장할 수 없다. 이때 레이스 컨디션이 발생하면 무작위한 프로세스 실행 순서에 의해 결과를 예측하기 어려워진다. 프로그램은 동일 입력에 대해 동일 출력을 보장해야 한다는 점을 고려하면, 이런 현상은 버그 또는 에러에 가까운 심각한 문제다.

레이스 컨디션의 근본적인 문제는 여러 프로세스가 동시에 같은 데이터에 접근하는 것이다. 이를 방지함으로써 크리티컬 섹션 문제를 방지할 수 있다. 만족해야 할 조건은 다음과 같다.

1. mutual exclusion(상호 배제): 프로세스가 크리티컬 섹션에 있다면, 다른 프로세스들의 진입을 막아 자원을 공유하는 상황을 방지한다. 기아 문제, 데드락 문제가 발생 가능
    - 기아(starvation): 프로세스가 무기한 대기하며, 자원을 할당 받지 못하는 상태
    - 데드락(dead lock): 여러 프로세스가 자원을 점유한 채로, 다른 프로세스가 가진 자원을 받을 때까지 무기한 대기하는 상태
2. progress(진행): 크리티컬 섹션이 비어 있고, 대기 중인 프로세스도 없다면, 나중에 크리티컬 섹션을 요구하는 어떤 프로세스가 왔을 때 진입할 수 있어야 한다.
3. bounded waiting(제한된 대기): 일단 프로세스가 크리티컬 섹션 대기열 안에 들어왔다면, 한정된 대기 횟수 안에 크리티컬 섹션에 진입할 수 있어야 한다. 기아 문제를 해결하는 방법. 

## Mutex&lt;T&gt;
lock은 있는데, unlock은 별도로 존재하지 않는다. lock() 메서드를 풀어서 받은 MutexGuard 변수가 스코프를 벗어나거나, 소유권을 잃어 해제되면 자동으로 unlock 상태가 된다.
```
m = Mutex { data: <locked>, poisoned: false, .. }
m = Mutex { data: 20, poisoned: false, .. }
```
락이 걸려 있는 경우 데이터에 직접 접근할 수 없다.
```rust
use std::sync::Mutex;

let m = Mutex::new(10);

{
    let mut num = m.lock().unwrap();
    *num += 10;

    drop(num); // 없으면 아래 lock 때문에 무한정 대기

    let mut num2 = m.lock().unwrap();
    *num2 += 20;
}

println!("m = {:?}", m);
```
- `Mutex::new`: 뮤텍스를 생성
- `Mutex::lock`: 락을 요청한다. 락이 걸릴 때까지 무한정 대기한다.
- `Mutex::try_lock`: 현재 락을 걸 수 있는지 검사한다.
- unlock: drop(num) 을 통해 소유권이 해제되면 unlock이 된다.

이때 m이 불변으로 지정되었지만, 내부의 값을 변경할 수 있음을 볼 수 있다. 이는 Mutex가 `RefCell<T>`처럼 **내부 가변성을 제공**하기 때문이다.


## 여러 스레드에서 뮤텍스 공유
`Mutex<T>`도 러스트의 일반적인 타입들과 마찬가지로 스레드의 클로저에 캡쳐되면 이동(move) 해야 한다. 자체적으로는 어떤 "공유" 기능도 없으므로, 스레드 간 공유를 위해서는 관련 포인터 박싱이 필요하다.

이때, 이전에 배웠던 `Rc<T>`는 여러 객체 사이에서 소유권을 공유할 수 있게 했으므로 이를 사용하는 방법을 생각할 수도 있겠지만, `Rc<T>`는 스레드 간 안전한 데이터 공유를 위한 어떤 보장도 하지 않는다. 즉, 멀티 스레드에 대응하려면 다른 스마트 포인터가 필요하다.

```rust
use std::sync::Mutex;
use std::thread;
use std::rc::Rc;


let counter = Rc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 1..10 {
    let counter = Rc::clone(&counter); 
    let handle = thread::spawn(|| {
        let mut num = counter.lock().unwrap(); // 이 부분이 문제
        *num += 1;
    });
    handles.push(handle);
}
```
위 코드에서 발생하는 에러 메시지는 아래와 같다. 요약하면 `Rc<T>`는 `Send` 트레잇을 구현하지 않으므로 안전하게 데이터를 공유할 수 없음을 의미한다.
```
`Rc<Mutex<i32>>` cannot be shared between threads safely
the trait `Sync` is not implemented for `Rc<Mutex<i32>>`
required for `&Rc<Mutex<i32>>` to implement `Send`
```
## Arc&lt;T&gt;
멀티 스레딩 환경에서 안전하게 소유권을 공유하기 위한 스마트 포인터로 `Arc<T>`를 제공한다.

이름은 Atomic + Rc로, 원자적으로 참조자를 센다는 의미를 가지고 있으며, 이를 통해 스레드 간 데이터를 공유하는 상황에도 안전하다.

모든 것이 Atomic하다면 더 안전하게 만들 수 있겠지만, 일반적 연산에 비해 성능 저하가 있으므로 스레드 안전성이 필요한 경우에만 Atomic 기반 기능을 이용한다.

```rust
use std::sync::Mutex;
    use std::thread;
    use std::sync::Arc;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", counter.lock().unwrap());
```

코드는 단순히 Rc를 Arc로 바꾸면 동작한다.

| 기능 | 싱글 스레드 | 멀티 스레드 | 
| - | - | - |
| 내부 가변성 | `RefCell<T>` | `Mutex<T>` |
| 다중 소유권 | `Rc<T>` | `Arc<T>` |

mutex 라이브러리는 근본적으로 상호 배제(mutual exclusion)을 제공하는 기능으로, 사용하기에 따라 기아, 데드락 등의 문제가 발생할 수 있어 주의가 필요하다. 

Arc도 downgrade을 통해 weak ptr을 얻을 수 있다.

# Sync & Send 트레잇
여러 스레드 간 안전하게 데이터를 공유하기 위해 필요한 트레잇으로 `std::marker` 트레이트인 `Sync`와 `Send`가 존재한다.

- `Send` 마커 트레잇: 스레드 사이 소유권 이동을 허용
- `Sync` 마커 트레잇: 여러 스레드 사이의 접근 허용

러스트의 대부분 타입은 `Send`이지만, 스레드 안정성·성능 저하 등의 이유로 `Send`가 적용되지 않는 타입이 존재한다. `Rc<T>`가 그 예시. 타입의 전체 요소가 `Send`라면, 해당 타입은 `Send`다.

`Sync`가 적용되면, 해당 타입은 여러 스레드로부터 안전하게 참조할 수 있음을 의미한다. 타입 T에 대한 불변 참조자 &T가 `Send`라 안전하게 다른 스레드로 보내질 수 있다면 `Sync`가 된다. 타입 전체가 `Sync`라면, 해당 타입은 `Sync`이다.

타입들에 대해 자동으로 `Send`, `Sync`가 적용되기 때문에 별도로 구현할 필요도 없고, 구현할 메서드도 존재하지 않는다. 따라서 이를 직접 구현하는 것은 신중해야 한다. (내부적으로 unsafe 코드를 이용해야 한다고 함)