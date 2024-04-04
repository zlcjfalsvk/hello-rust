use crate::r#struct::User;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::Read;

mod r#struct;

// Json 을 struct 로 converting 해보자
fn main() -> Result<(), io::Error> {
    parse_test()?;

    Ok(())
}

fn parse_test() -> Result<Vec<User>, io::Error> {
    let mut jsonStr = String::new();
    File::open("s_json_parser/data/parsor_test.json")?
        .read_to_string(&mut jsonStr)?
        .to_string();

    let converted_json: Vec<User> = serde_json::from_str(&jsonStr)?;

    Ok(converted_json)
}
