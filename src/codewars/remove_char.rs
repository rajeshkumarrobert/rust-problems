pub fn remove_char(s: &str) -> String {

    // Your code here!
    //String::from("Code on, rustacean!");
    let mut result =String::new();
    let length = s.len();
    println!("Length:{length}");
    for (i,value) in s.chars().enumerate(){
        if i == 0 || i == length-1{
            println!("{i},{value}");
            continue;
        }else {
            result.push(value);
        }
    }
    result
    //s[1..s.len() - 1].to_string()
}
