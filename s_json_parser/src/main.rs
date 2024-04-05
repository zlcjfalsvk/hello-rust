use crate::r#struct::User;

use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::Read;

mod r#struct;

const JSON_CHILDREN_KEY: &str = "children";

// Json 을 struct 로 converting 해보자
fn main() -> Result<(), io::Error> {
    let _pu = parse_test(String::from("s_json_parser/data/parser_test.json").as_str())?;
    let _njpu = nested_json_parse_test(
        String::from("s_json_parser/data/nested_parser_test.json").as_str(),
    )?;

    assert_eq!(_pu.len(), _njpu.len());

    Ok(())
}

fn parse_test(path: &str) -> Result<Vec<User>, io::Error> {
    let json_str = read_file(path)?;
    let converted_json: Vec<User> = serde_json::from_str(&json_str)?;

    Ok(converted_json)
}

fn nested_json_parse_test(path: &str) -> Result<Vec<User>, io::Error> {
    let json_str = read_file(path)?;
    let mut converted_json: Value = serde_json::from_str(&json_str)?;
    let mut user_vec: Vec<User> = Vec::new();

    start_nested_json_parse_test(&mut user_vec, &mut converted_json);

    Ok(user_vec)
}

fn start_nested_json_parse_test<'a>(vec: &'a mut Vec<User>, json: &Value) -> &'a mut Vec<User> {
    let mut map = json.as_object().unwrap().clone();
    map.remove(JSON_CHILDREN_KEY);
    // generate user
    let user = User::from(map);
    // push user to vec
    vec.push(user);

    let has_children = json.get(JSON_CHILDREN_KEY);
    if has_children.is_none() {
        return vec;
    }

    let children: Vec<Value> = has_children.unwrap().as_array().unwrap().clone();
    for item in &children {
        start_nested_json_parse_test(vec, item);
    }

    vec
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut json_str = String::new();
    File::open(path)?.read_to_string(&mut json_str)?.to_string();
    Ok(json_str)
}
