# Node 

- rust 는 try-catch statement 없음
  - 대신 다음과 같이 사용 가능 (? operator) 
    - [Link](https://stackoverflow.com/questions/55755552/what-is-the-rust-equivalent-to-a-try-catch-statement)

- Error 처리시 `?` shortcut 은 from 함수를 거친다
  - From trait 에 정의되어 있으며 어떤 값의 타입을 다른 타입으로 변환 하는데 사용