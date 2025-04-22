pub fn merge_alternately(word1: String, word2: String)->String{
let mut result = String::new();
let mut chars1 = word1.chars();
let mut chars2 = word2.chars();

loop {
    match (chars1.next(), chars2.next()) {
        (Some(c1), Some(c2)) => {
            result.push(c1);
            result.push(c2);
        }
        (Some(c1), None) => result.push(c1),
        (None, Some(c2)) => result.push(c2),
        (None, None) => break,
    }
}

result
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let mut result = String::new();
    let  chars1 = str1.chars();
    let  chars2 = str2.chars();
    let mut s1 = str1.clone();
    s1 += &str2.clone();

    let mut s2 = str2.clone();
    s2 += &str1.clone();

    if s1 != s2 {
        return String::from("");
    }
    chars1.zip(chars2).for_each(|(c1, c2)| {
        if c1 == c2 {
            if !result.contains(c1){
                result.push(c1);
            }
            
        }
    });
    result   
}