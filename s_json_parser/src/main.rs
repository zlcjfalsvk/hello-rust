use crate::r#struct::User;

use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::Read;

mod r#struct;

// Json 을 struct 로 converting 해보자
fn main() -> Result<(), io::Error> {
    let pu = parse_test(String::from("s_json_parser/data/parser_test.json").as_str())?;
    let njpu = nested_json_parse_test(
        String::from("s_json_parser/data/nested_parser_test.json").as_str(),
    )?;

    assert_eq!(pu.len(), njpu.len());

    Ok(())
}

fn parse_test(path: &str) -> Result<Vec<User>, io::Error> {
    let jsonStr = read_file(path)?;
    let converted_json: Vec<User> = serde_json::from_str(&jsonStr)?;

    Ok(converted_json)
}

fn nested_json_parse_test(path: &str) -> Result<Vec<User>, io::Error> {
    let jsonStr = read_file(path)?;
    let converted_json: Value = serde_json::from_str(&jsonStr)?;

    let userVec: Vec<User> = Vec::new();

    Ok(userVec)
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut jsonStr = String::new();
    File::open(path)?.read_to_string(&mut jsonStr)?.to_string();
    Ok(jsonStr)
}
