pub fn is_valid(s: String) -> bool {
      let bracket_map = [(')', '('), ('}', '{'), (']', '[')].iter().cloned().collect::<std::collections::HashMap<_, _>>();
        let mut stack = Vec::new();

        for c in s.chars() {
            match bracket_map.get(&c) {
                Some(&open_bracket) => {
                    if stack.pop() != Some(open_bracket) {
                        return false;
                    }
                }
                None => { stack.push(c); }
            }
        }
        stack.is_empty()  
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
      if strs.len() == 1 {
            return strs[0].clone();
        }
        
        let short_string = strs.iter().min().unwrap();
        println!("short string:{}",short_string);
        let long_string = strs.iter().max().unwrap();
        println!("long string:{}",long_string);
        let short_bytes = short_string.as_bytes();
        let long_bytes = long_string.as_bytes();

        let mut prefix_end = 0;
        for i in 0..short_bytes.len() {
            if short_bytes[i] != long_bytes[i] {
                break;
            }
            prefix_end += 1;
        }

        short_string[..prefix_end].to_string()
}

pub fn roman_to_int(s: String) -> i32 {
       let s_translated = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        s_translated.chars().map(|c| {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }).sum()
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let h_len = haystack.len();
    let n_len = needle.len();

    if h_len < n_len {
        return -1;
    }

    for front in 0..=(h_len - n_len) {

        if haystack[front..front+n_len] == needle {
            return front as i32;
        }
    }
    -1
}

pub fn length_of_last_word(s: String) -> i32 {
    let last_word:Vec<&str> = s.split_whitespace().collect();
    let length = last_word.last().unwrap().chars().count();
    length as i32
}