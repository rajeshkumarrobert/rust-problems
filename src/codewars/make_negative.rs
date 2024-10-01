pub fn make_negative(n: i32) -> i32 {
    if n.is_negative(){
        n
    }else if n==0 {
        0
    }else{
        -n
    }
}