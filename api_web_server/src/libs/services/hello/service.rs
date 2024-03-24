pub fn greeting(name: &str) -> String {
    let mut greeting: String = String::from("Hello ");
    greeting.push_str(name);
    greeting.push('!');

    greeting
}
