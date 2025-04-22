pub fn opposite(number: i32) -> i32 {
    if number.is_positive(){
        return -number;
    }else {
        return number.abs();
    }
}