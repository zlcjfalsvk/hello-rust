// 데이터 압축 및 해제 학습
// miniz_oxide 라이브러리를 사용한 DEFLATE 압축 알고리즘 구현

use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;

// 압축과 해제를 테스트하는 함수 (Round-trip test)
// 원본 데이터 -> 압축 -> 해제 -> 원본과 비교
fn roundtrip(data: &[u8]) {
    println!("data: {:?}", data);
    
    // 데이터 압축
    // 두 번째 매개변수 6은 압축 레벨 (0-10, 6이 기본값)
    // 높은 값일수록 압축률은 높아지지만 속도는 느려짐
    let compressed = compress_to_vec(data, 6);
    println!("compressed data: {:?}", &compressed);

    // 압축된 데이터 해제
    // as_slice()를 사용하여 Vec을 슬라이스로 변환
    // expect()로 에러 처리 - 해제 실패 시 패닉 발생
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
    println!("decompressed data: {:?}", &decompressed);

    // 원본 데이터와 해제된 데이터가 일치하는지 검증
    // assert_eq!는 두 값이 같지 않으면 패닉 발생
    assert_eq!(data, decompressed);
}

fn main() {
    // 문자열을 바이트 배열로 변환
    // as_bytes()는 &str을 &[u8]로 변환
    let message_bytes = "Hello, world!".as_bytes();
    
    // 압축-해제 테스트 실행
    roundtrip(message_bytes);
    
    // 성공적으로 실행되면 압축과 해제가 정상적으로 작동함을 의미
}
