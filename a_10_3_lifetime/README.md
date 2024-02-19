# Note

- lifetime 의 주목적은 dangling reference 방지
  - dangling reference 는 원하는 데이터의 참조가 아닌 다른 데이터를 참조하게 되는 원인

- lifetime 을 명시한다고 참조자의 수명이 바뀌진 않음, 참조자에 대한 lifetime 간의 관계가 어떻게 되는지 기술하고 rust 에게 알려주는 용도

- lifetime 선언 생략이 가능함 [Doc](https://doc.rust-kr.org/ch10-03-lifetime-syntax.html#%EB%9D%BC%EC%9D%B4%ED%94%84%ED%83%80%EC%9E%84-%EC%83%9D%EB%9E%B5)
  - 규칙 1
    - 컴파일러가 참조자인 매개변수 각각에게 라이프타임 매개변수를 할당
  - 규칙 2
    - 입력 라이프타임 매개변수가 딱 하나라면, 해당 라이프타임이 모든 출력 라이프타임에 대입
    - ex) `fn foo<'a>(x: &'a i32) -> &'a i32`
  - 규칙 3
    - 메서드 시그니처에만 적용
    - `impl` 에 lifetime 명시 하면 메서드에 생략 가능

- static lifetime `'static` 은 프로그램의 전체 생애주기 동안 살아있음
  - 모든 문자열 literal 은 `'static` lifetime 을 가진다
  - ex) `let s: &'static str = "I have a static lifetime.";` 