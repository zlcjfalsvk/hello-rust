# Note

- closure
  - 변수에 저장하거나 다른 함수에 인수로 전달할 수 있는 익명 함수
  - 정의도니 스코프에서 값을 캡처

- closure 가 소유권이 필요할 경우 param list 전에 `move` 키워드 사용 가능

- closure 는 `Fn`,`FnOnce`,`FnMut` 중 선택 (여러개 가능) 구현