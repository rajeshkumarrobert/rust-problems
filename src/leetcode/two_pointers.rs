pub fn reverse_str(s: String, k: i32) -> String {
    let len = s.len() as i32;
    if k > len{
        return s.chars().rev().collect::<String>();
    }
    let char_vec = s.chars().collect::<Vec<_>>();
    let temp = 2 * k;
    let grouped: Vec<Vec<char>> = char_vec
        .chunks(temp as usize)
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut res = Vec::new();
    for mut slice in grouped{
        match slice.split_at_mut_checked(k as usize) {
            Some((left, right)) => {
                left.reverse();
                res.push([left, right].concat());
            }
            None => {
                // If split_at_mut_checked returns None, just reverse the whole slice
                slice.reverse();
                res.push(slice);
            }
        }
    }
        res.iter().map(|x| x.iter().collect::<String>()).collect::<String>() 
    }

pub fn reverse_words(s: String) -> String {
        let vec_words = s.split_whitespace().map(|a| a.to_string()).collect::<Vec<_>>();
        let mut res = String::new();
        for word in vec_words{
            res.push_str(&word.chars().rev().collect::<String>().as_str());
            res.push(' ');
        }
        res.trim().to_string()
    }

pub fn valid_palindrome(s: String) -> bool {
        let s_chars = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = s_chars.len()-1;
        let mut res = true;
        let mut temp = 0;
        while left<right {
           if s_chars[left] == s_chars[right]{
                res = true;
           }else if (s_chars[left+1] == s_chars[right]) && temp!=1{
                res = true;
                temp += 1;
                left+=1;
                continue;
           }else if(s_chars[left] == s_chars[right-1])&& temp!=1{
               res = true;
               temp += 1;
               right-=1;
               continue;
           }else {
               res = false;
               break;
           }  
           left+=1;
           right-=1;
        }
        res
    }