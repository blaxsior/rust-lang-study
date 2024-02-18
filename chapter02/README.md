# Guessing Game

## 표준 라이브러리
std 이하에 표준 라이브러리들이 선언되어 있다.

일부 기능들은 명시하지 않아도 자동으로 로딩한다. [rust::prelude](https://doc.rust-lang.org/std/prelude/index.html)을 참고.

라이브러리에 포함된 기능은 ```use std::libname::{something}``` 을 입력해서 로딩한다.  

## 의존성 추가
두가지 방법 중 하나로 의존성을 추가할 수 있음
1. ```cargo add 의존성_이름```으로 등록
2. Cargo.lock의 dependencies 부분에 추가

이후 ```cargo build``` 을 통해 의존성을 빌드

## 사용되는 문법 요소
- match: 스위치 문을 표현식(반환 값 O)으로 다룰 수 있음
- let, mut: 변수 선언 시 let 키워드 사용. 변경해야 하면 mut 키워드 필요