pub fn greet(name: &str) -> String {
    let result = String::from("Hello, <string> how are you doing today?");
    result.replace("<string>", name)
    //format!("Hello, {name} how are you doing today?")
}