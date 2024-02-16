fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec!["1", "2", "3"];
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // 3
    let third: Option<&i32> = v.get(2); // 3
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // let does_not_exist = &v[100]; // panic 처리 해야 함
    // let does_not_exist = v.get(100); // None


    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}"); // ownership issue

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    } // [150, 82, 107]

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}