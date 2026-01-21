use core::f64;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut res: f64 = f64::MIN;
    for x in 0..=nums.len() as i32 - k {
        let total: i32 = nums[x as usize..x as usize + k as usize].iter().sum();
        res = res.max(total as f64 / k as f64);
    }
    res
}

pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let n = code.len();
    match k {
        k if k > 0 => {
            for i in 0..n {
                let mut sum = 0;
                let mut count = k;
                let mut j = (i + 1) % n;
                while count > 0 {
                    sum += code[j];
                    j = (j + 1) % n;
                    count -= 1;
                }
                res.push(sum);
            }
        }
        0 => {
            for _i in 0..n {
                res.push(0);
            }
        }
        k if k < 0 => {
            for i in 0..n {
                let mut sum = 0;
                let mut count = k.abs();
                let mut j = (i + n - 1) % n;
                while count > 0 {
                    sum += code[j];
                    j = (j + n - 1) % n;
                    count -= 1;
                }
                res.push(sum);
            }
        }
        _ => {}
    }
    res
}

pub fn longest_nice_substring(s: String) -> String {
    let mut hs = HashSet::new();
    for ch in s.chars().into_iter() {
        hs.insert(ch.to_string());
    }

    for (index, ch) in s.chars().into_iter().enumerate() {
        match ch.is_uppercase() {
            true => {
                if hs.contains(&ch.to_lowercase().to_string()) {
                    continue;
                }
            }
            false => {
                if hs.contains(&ch.to_uppercase().to_string()) {
                    continue;
                }
            }
        }
        let string_left = longest_nice_substring(s[..index].to_string());
        let string_right = longest_nice_substring(s[index + 1..s.len()].to_string());

        match string_left.len().cmp(&string_right.len()) {
            Ordering::Greater | Ordering::Equal => return string_left,
            _ => return string_right,
        }
    }
    return s;
}

pub fn count_good_substrings(s: String) -> i32 {
    let binding = s.chars().collect::<Vec<_>>();
    let s_char = binding.windows(3).collect::<Vec<_>>();
    let mut count = 0;
    for value in s_char {
        if value[0] != value[1] && value[1] != value[2] && value[0] != value[2] {
            count += 1;
        }
    }
    count
}

pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    nums.windows(k as usize)
        .map(|pair| pair[(k - 1) as usize] - pair[0])
        .min()
        .unwrap()
}

pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    let string_num = num.to_string().chars().into_iter().collect::<Vec<_>>();
    let mut res = 0;
    for val in string_num.windows(k as usize) {
        // collect the slice of chars into a String and parse to i32
        let substring: String = val.iter().collect();
        let val_num = substring.parse::<i32>().unwrap_or(0);
        if val_num != 0 && num % val_num == 0 {
            res += 1;
        }
    }
    res
}

pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let k_blocks = blocks.chars().into_iter().collect::<Vec<_>>();
    let mut res = vec![];
    for block in k_blocks.windows(k as usize) {
        let mut counter = 0;
        for c in block {
            match c {
                'W' => counter += 1,
                _ => (),
            }
        }
        res.push(counter);
    }
    *res.iter().min().unwrap()
}

pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
    fn check_condition(range: &[i32], threshold: i32) -> i32 {
        let n = range.len();
        if n == 0 {
            return 0;
        }
        // First element must be even and not exceed threshold.
        if (range[0].abs() % 2 != 0) || range[0] > threshold {
            return 0;
        }
        let mut count = 1;
        for i in 1..n {
            if range[i] > threshold {
                break;
            }
            // parity of current must differ from previous (alternating).
            if (range[i].abs() % 2) == (range[i - 1].abs() % 2) {
                break;
            }
            count += 1;
        }
        count
    }

    let mut best = 0;
    for i in 0..nums.len() {
        best = best.max(check_condition(&nums[i..], threshold));
    }
    best
}

pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
    let mut res = 0;
        for x in &nums{
            for y in &nums{
                if (x-y).abs() <= (*x.min(&y)){
                    res = res.max(x ^ y);
                }
            }
        }
    res
    }

pub fn maximum_length_substring(s: String) -> i32 {
    let mut freq = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut max_len = 0;

    for right in 0..chars.len() {
        *freq.entry(chars[right]).or_insert(0) += 1;

        // shrink window if any character appears more than 2 times
        while freq[&chars[right]] > 2 {
            let left_char = chars[left];
            *freq.get_mut(&left_char).unwrap() -= 1;
            left += 1;
        }

        max_len = max_len.max((right - left + 1) as i32);
    }

    max_len
    }

pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = usize::MAX;
    for i in 0 .. nums.len() {
      let mut curr = 0;
      for j in i .. nums.len() {
        curr |= nums[j];
        if curr >= k {
          res = res.min(j - i + 1);
          break;
        }
      }
    }
    
    if res == usize::MAX {
      return -1;
    }
    return res as i32;
    }

pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let mut res = 0;
    let first = colors[0];
    let second = colors[1];
    let last = *colors.last().unwrap();
    let last_prev = *colors.get(colors.len()-2).unwrap();

    for range in colors.windows(3){
        if (range[0] != range[1]) && (range[1] != range[2]){
            res+=1;
        }
    }
    
    if (last != first) && (first != second){
        res+=1;
    }

    if (last_prev != last) && (last != first){
        res +=1;
    }

    res
    }

//medium
pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash: HashMap<char, i32> = HashMap::new();
        let mut ans = 0;
        let mut lo = -1;
        for (hi, ch) in s.chars().enumerate() {
            if let Some(i) = hash.insert(ch, hi as i32) {
                lo = lo.max(i);
            }
            ans = ans.max(hi as i32 - lo);
        }
        ans
    }

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let mut res = vec![];
    let vec_collection = s.chars().collect::<Vec<_>>();
    for substring in vec_collection.windows(10){
        res.push(substring.iter().collect::<String>());
    }
    let mut freq: HashMap<String, usize> = HashMap::new();

    for word in res {
        *freq.entry(word).or_insert(0) += 1;
    }

    freq.into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(word, _)| word)
        .collect()
    }

pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut max_length = 0;
        let chars = s.chars().collect::<Vec<char>>();
        for i in 0..s.len(){
            let mut map = HashMap::new();
            for j in i..s.len(){
                let count = map.entry(chars[j]).or_insert(0);
                *count+=1;

            if map.iter().all(|(_,count)| *count >= k){
                max_length = max_length.max(j-i +1);
            } 
            }
        }
        return max_length as i32;
    }

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut res = 0;
        for i in 3..=nums.len(){
            for slice in nums.windows(i){
                let d = slice[1] - slice[0];
                let mut valid = true;
                for x in 2..slice.len(){
                    if slice[x] - slice[x-1] != d{
                        valid = false;
                        break;
                    }
                }
                if valid{
                    res+=1;
                }
            }
        }
    res
    }  