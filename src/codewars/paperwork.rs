pub fn paperwork(n: i16, m: i16) -> i16 {
    if n.is_negative() || m.is_negative(){
        return 0;
    }else{
        n*m
    }
}