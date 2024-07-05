use json_patch::diff;
use serde_json::{json, Value};

fn main() {
    json_patch();
}

fn json_patch() {
    let p = diff(&get_json_original(), &get_json_changed());

    // Patch([Remove(RemoveOperation { path: "/author/familyName" }),
    // Add(AddOperation { path: "/phoneNumber", value: String("+01-123-456-7890") }),
    // Remove(RemoveOperation { path: "/tags/1" }),
    // Replace(ReplaceOperation { path: "/title", value: String("Hello!") })])
    println!("{:?}", p);
}

fn get_json_original() -> Value {
    json!({
      "title": "Goodbye!",
      "author" : {
        "givenName" : "John",
        "familyName" : "Doe"
      },
      "tags":[ "example", "sample" ],
      "content": "This will be unchanged"
    })
}

fn get_json_changed() -> Value {
    json!({
      "title": "Hello!",
      "author" : {
        "givenName" : "John"
      },
      "tags": [ "example" ],
      "content": "This will be unchanged",
      "phoneNumber": "+01-123-456-7890"
    })
}
