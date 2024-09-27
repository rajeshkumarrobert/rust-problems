pub fn digitize(n:i64)->Vec<i8>{
    let mut result:Vec<i8> = Vec::new();
    let mut num = n;
    while num>0 {
        result.push((num%10) as i8);
        num /= 10;    
    }
    if result.is_empty(){
        result.push(0);
    }
    println!("{:?}",result);
    result

}