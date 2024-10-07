pub fn square_sum(vec: Vec<i32>) -> i32 {
     let res:Vec<i32> =  vec.iter().map(|x| x*x).collect();
     res.iter().fold(0, |acc, x| acc+x)
     //vec.iter().map(|s| s * s).sum()
}