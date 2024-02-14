
# Note

- scalar type(Integer, Float, Boolean, Char)
  - Integer Type 중 `arch (usize: unsigned)`는 os 환경마다 유동적인 값을 가짐
    - ex) 64-bit 아키텍처 -> 64bit, 32-bit 아키텍처 -> 32bit
  - c 언어 처럼 문자열은 `""` 문자는 `''` 사용

- compound type (tuple, array)

- 반복문
  - loop
  - while
    - 다른 lang 과 다르게 `do-while` 없음 (매크로 만들어서 사용 가능)
  - for
    - 다른 lang 과 다르게 `for-of` 나 `for(;;)` 없음