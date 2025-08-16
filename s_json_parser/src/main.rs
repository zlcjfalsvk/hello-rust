// JSON 파싱 및 직렬화/역직렬화 학습
// serde_json 라이브러리를 사용한 JSON 처리

use crate::r#struct::User;  // r#struct는 예약어 struct를 모듈명으로 사용하기 위한 문법

use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::{io, thread};

mod r#struct;

// 중첩된 JSON 구조에서 자식 노드를 나타내는 키
const JSON_CHILDREN_KEY: &str = "children";

fn main() -> Result<(), io::Error> {
    // 각각 다른 파싱 방법을 테스트
    
    // 1. 기본 JSON 배열 파싱 - User 구조체 배열로 변환
    let _pu = parse_test(String::from("s_json_parser/data/parser_test.json").as_str())?;

    // 2. 중첩된 JSON 파싱 - 트리 구조를 평탄화하여 User 배열로 변환
    let _njpu = nested_json_parse_test(
        String::from("s_json_parser/data/nested_parser_test.json").as_str(),
    )?;

    // 3. HashMap으로 파싱 - 동적 키-값 처리
    let _pu2 = parse_test_2(String::from("s_json_parser/data/parser_test_2.json").as_str())?;

    // 테스트 검증: 두 방법으로 파싱한 결과가 같은지 확인
    assert_eq!(_pu.len(), _njpu.len());
    Ok(())
}

// 기본 JSON 파싱: JSON 배열을 User 구조체 Vec로 직접 변환
fn parse_test(path: &str) -> Result<Vec<User>, io::Error> {
    let json_str = read_file(path)?;
    // serde_json::from_str은 JSON 문자열을 Rust 타입으로 자동 변환
    let converted_json: Vec<User> = serde_json::from_str(&json_str)?;

    Ok(converted_json)
}

// 동적 JSON 파싱: HashMap을 사용하여 유연한 키-값 처리
// JSON 객체의 키들을 동적으로 조회하고 처리
fn parse_test_2(path: &str) -> Result<(), io::Error> {
    let json_str = read_file(path)?;
    // HashMap으로 파싱하여 동적으로 키 접근 가능
    let converted_json: HashMap<String, Value> = serde_json::from_str(&json_str)?;

    // "obj" 키의 값을 가져와서 객체로 변환 후 모든 키-값 출력
    converted_json
        .get("obj")           // "obj" 키의 값 가져오기
        .unwrap()            // Option 해제
        .as_object()         // Value를 JSON 객체로 변환
        .unwrap()
        .iter()              // 모든 키-값 순회
        .for_each(|(k, v)| {
            println!("{k}: {v}");
            println!("----------------------");
        });

    Ok(())
}

// 중첩된 JSON 파싱: 트리 구조를 순회하며 User 추출
fn nested_json_parse_test(path: &str) -> Result<Vec<User>, io::Error> {
    let json_str = read_file(path)?;
    // Value 타입으로 파싱하여 동적 처리
    let mut converted_json: Value = serde_json::from_str(&json_str)?;
    let mut user_vec: Vec<User> = Vec::new();

    // 재귀적으로 트리 구조 순회
    start_nested_json_parse_test(&mut user_vec, &mut converted_json);

    Ok(user_vec)
}

// 재귀 함수: 중첩된 JSON에서 모든 User 추출
// 'a는 라이프타임 파라미터 - 참조자의 생명주기를 명시
fn start_nested_json_parse_test<'a>(vec: &'a mut Vec<User>, json: &Value) -> &'a mut Vec<User> {
    // 현재 노드에서 User 정보 추출
    let mut map = json.as_object().unwrap().clone();
    map.remove(JSON_CHILDREN_KEY);  // children 필드 제거 (순수 User 데이터만 남김)
    
    // Map을 User 구조체로 변환
    let user = User::from(map);
    vec.push(user);

    // 자식 노드 처리
    let has_children = json.get(JSON_CHILDREN_KEY);
    if has_children.is_none() {
        return vec;  // 자식이 없으면 종료
    }

    // 모든 자식 노드에 대해 재귀 호출
    let children: Vec<Value> = has_children.unwrap().as_array().unwrap().clone();
    for item in &children {
        start_nested_json_parse_test(vec, item);
    }

    vec
}

// 스레드를 사용한 병렬 처리 버전 (미완성 - 데이터 경합 문제 있음)
// 주의: 여러 스레드가 동시에 Vec에 접근하므로 동기화 필요
fn start_nested_json_parse_test_thread<'a>(
    vec: &'a mut Vec<User>,
    json: &Value,
) -> &'a mut Vec<User> {
    let mut map = json.as_object().unwrap().clone();
    map.remove(JSON_CHILDREN_KEY);
    let user = User::from(map);
    vec.push(user);

    let has_children = json.get(JSON_CHILDREN_KEY);
    if has_children.is_none() {
        return vec;
    }

    let children: Vec<Value> = has_children.unwrap().as_array().unwrap().clone();
    for item in &children {
        // thread::scope를 사용하여 스레드 생성
        // 주의: 현재 코드는 컴파일 에러 발생 (가변 참조 동기화 문제)
        thread::scope(|s| {
            s.spawn(|| start_nested_json_parse_test_thread(vec, item));
        });
    }
    vec
}

// 파일에서 JSON 문자열 읽기
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut json_str = String::new();
    // ?는 에러 전파 연산자 - 에러 발생 시 즉시 리턴
    File::open(path)?.read_to_string(&mut json_str)?;
    Ok(json_str)
}
