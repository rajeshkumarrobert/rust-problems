use std::{collections::{HashMap, HashSet}};

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

pub fn is_palindrome(s: String) -> bool {
    let str_value:String = s.chars().filter(|c| c.is_alphanumeric()).collect::<Vec<char>>().iter().collect();
    let lower_str = str_value.to_ascii_lowercase();
    lower_str.chars().eq(lower_str.chars().rev())
}

pub fn convert_to_title(column_number: i32) -> String {
    let mut res = String::from("");
    let mut column_number = column_number;
    while column_number > 0 {
        column_number -=1;
        res.push(((column_number%26) as u8 + b'A')as char);
        column_number/=26;
    }
    res.chars().rev().collect()
}

pub fn title_to_number(column_title: String) -> i32 {
        let chars = column_title.chars().collect::<Vec<char>>();
        let mut char_length = chars.len() as u32;
        let value = |c| match c {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            'D' => 4,
            'E' => 5,
            'F' => 6,
            'G' => 7,
            'H' => 8,
            'I' => 9,
            'J' => 10,
            'K' => 11,
            'L' => 12,
            'M' => 13,
            'N' => 14,
            'O' => 15,
            'P' => 16,
            'Q' => 17,
            'R' => 18,
            'S' => 19,
            'T' => 20,
            'U' => 21,
            'V' => 22,
            'W' => 23,
            'X' => 24,
            'Y' => 25,
            'Z' => 26,
            _ => 0,
        };
        let mut result = 0;
        let base:i32 = 26;
        for c in chars{
            char_length -=1;
            result += value(c) * (base.pow(char_length));
            println!("{result}, char length:{}, length:{}",char_length,(base.pow(char_length)));
            
        }
        result
}

pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        help(s, t) && help(t, s)
    }

    fn help(s: &[u8], t: &[u8]) -> bool {
        let mut map = HashMap::new();
        for i in 0..s.len() {
            if map.contains_key(&s[i]) {
                if map.get(&s[i]).unwrap() != &t[i] {
                    return false;
                }
            } else {
                map.insert(s[i], t[i]);
            }
        }
        true
    }

pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_char = s.chars().collect::<Vec<char>>();
        s_char.sort();
        let mut t_char = t.chars().collect::<Vec<char>>();
        t_char.sort();

        s_char.eq(&t_char)
    }

pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern_value = pattern.chars().collect::<Vec<char>>();
        let s_value = s.split_whitespace().collect::<Vec<&str>>();
        let mut map = HashMap::new();
        if pattern_value.len() != s_value.len(){
            return false;
        }
        for i in 0..s_value.len(){
            let val = map.entry(pattern_value[i]).or_insert(s_value[i]);

            if *val != s_value[i]{
                return false;
            }
        }
        map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()
    }

pub fn reverse_string(s: &mut Vec<char>) {
        let mut s_len = s.len()-1;
        let mut left = 0;
        while left < s_len {
            s.swap(left, s_len);
            left+=1;
            s_len-=1;
        }
    }

pub fn reverse_vowels(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut s_len = s.len()-1;
        let mut left = 0;
        let res = |a| match a{
            'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => true,
            _ => false,
        };
        while left < s_len {
            while left < s_len && !res(s[left]) {
                left+=1; 
            }
            while left < s_len && !res(s[s_len]) {
                 s_len-=1;
            }
            if left < s_len{
                s.swap(left, s_len);      
                s_len-=1;
                left+=1;
            } 
        }
        s.iter().collect::<String>()
    }

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() <= magazine.len() {
            let mut count_chars_magazine = HashMap::new();
            for ch in magazine.chars() {
                count_chars_magazine
                    .entry(ch)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            for ch in ransom_note.chars() {
                match count_chars_magazine.get_mut(&ch) {
                    Some(c) if *c > 0 => {*c -= 1},
                    _ => {return false}
                }
            }
            true
        } else {
            false
        }
    }

pub fn first_uniq_char(s: String) -> i32 {
        let vec_char = s.chars().collect::<Vec<char>>();
        let mut val = HashMap::new();
        for char in vec_char.clone(){
            val.entry(char).and_modify(|c| *c+=1).or_insert(1);
        }
        println!("Hashmap ={:?}",val);
        let mut not_repeated_char:Option<char>=  None;
        
        for value in s.chars(){
            if *val.get(&value).unwrap() == 1{
                println!("{value:?}");
                not_repeated_char = Some(value);
                break;
            }
        }

        let mut not_repeated_index =0;
        let mut char_index =  |c:char| {
            for (i,char) in vec_char.iter().enumerate() {
                if c == *char{
                    not_repeated_index=i;
                    break;
                }
            }
        };
        if not_repeated_char.is_none(){
            return -1;
        }else {
            char_index(not_repeated_char.unwrap());
            return not_repeated_index as i32;
        }
    }

pub fn find_the_difference(s: String, t: String) -> char {
        let mut counts = [0u16; 26];
        for &ch in t.as_bytes() {
            counts[(ch - b'a') as usize] += 1;
        }
        for &ch in s.as_bytes() {
            counts[(ch - b'a') as usize] -= 1;
        }
        let pos = counts.iter().position(|&count| count != 0).unwrap();
        (pos as u8 + b'a') as _
    }

pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0{
            return true;
        }
        if t.len() < s.len(){
            return false;
        }
        let s_chars = s.chars().collect::<Vec<char>>();
        let t_chars = t.chars().collect::<Vec<char>>();
        let mut total_letters = s_chars.len();
        let mut left = 0;
        let right = t.len()-1;
        for c in s_chars{
            while left < right+1 {
                if c == t_chars[left]{
                    total_letters -=1;
                    left+=1;
                    break;
                }
                left+=1;
            }
        }
        println!("{total_letters}");
        if total_letters == 0{
            return true;
        }else {
            return false;
        }
    }

pub fn longest_palindrome(s: String) -> i32 {
        let s_chars = s.chars().collect::<Vec<char>>();
        let mut hash_table = HashMap::new();
        for c in s_chars{
            hash_table.entry(c).and_modify(|c| {*c+=1}).or_insert(1);
        }
        let mut result = 0;
        println!("{hash_table:?}");
        let mut y:i32=0;
        for val in hash_table.values(){
            if *val%2 ==0 {
                result = result+*val;
            }else {
                if *val>2{
                    let x = val/2;
                    result = result + 2*x;
                    y = val%2;
                }else{
                    y +=1;
                }

            } 
        }
        result + ( if y != 0 { 1 } else { 0 } )
    }

pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for i in 1..=n{
            if i%3 == 0 && i%5 == 0 {
                result.push("FizzBuzz".to_string());
            }else if i%5 == 0 {
                result.push("Buzz".to_string());
            }else if i%3 == 0{
                result.push("Fizz".to_string());
            }else {
                result.push(i.to_string());
            }
        }
        result
    }