# Node

- ownership, pointer, slice 는 컴파일 타임에 메모리 안정성을 보장하기 위함 

- 변수가 스코프 밖으로 벗어나면 `drop` 함수 자동 호출

- `&` 를 붙인 구문은 해당 값을 참조하지만 소유하지는 않는 참조사 생성

- dangling pointer + 라이프타임
  - `&'static`

- slice
  - collection 을 통째로 참조하는 것이 아닌, collection 의 연속된 일련의 요소 참조
  - 소유권 갖지 않음

  
- `&str`, `&String` 차이
  - `&str` 는 수정할 수 없는 읽기 전용 문자열 (immutable reference)
  - `&String` mutable 함