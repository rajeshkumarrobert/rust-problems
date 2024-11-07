pub fn high_and_low(numbers: &str) -> String {
    let mut out = String::new();
    let mut result:Vec<i32>= numbers.split(" ").map(|x| x.parse().unwrap()).collect();
    result.sort();
    println!("{:?}",result);
    let last_val = result.len()-1;
    out.push_str(result.get(last_val).unwrap().to_string().as_str());
    out.push_str(" ");
    out.push_str(result.get(0).unwrap().to_string().as_str());
    out
  }