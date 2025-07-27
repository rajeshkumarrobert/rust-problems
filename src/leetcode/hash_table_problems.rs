use std::collections::{HashMap, HashSet};

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut left = 0;
        let max = |number1:Vec<i32>, number2:Vec<i32>| {
            if number1.len() > number2.len(){
                return (number2,number1);
            }else {
                return (number1,number2);
            }
        };
        let (small,mut big) = max(nums1,nums2);
        println!("({:?},{:?})", small,big);
        let min_vec = small.clone();
        while left<small.len() {
            println!("{:?}",min_vec[left]);
            if big.contains(&min_vec[left]){
                let index = big.iter().position(|x| *x == min_vec[left]).unwrap();
                big.swap_remove(index);
                res.push(min_vec[left]);
            }
            left+=1;
        }
    res
    }

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        // let mut res = vec![];
        // let mut numbers = nums;
        // numbers.sort();
        // let mut index = 0;
        // for i in 1..=numbers.len() as i32{
        //     if index<numbers.len(){
        //         if i!=numbers[index as usize] && !numbers.contains(&i){
        //             res.push(i);
        //         }
        //     }else {
        //         break;
        //     }
        //     index+=1;
        // }
        // res
        let n = nums.len() as i32;
        let mut set = HashSet::new();
        for n in nums {
            set.insert(n);
        }
        (1..=n).filter(|v| !set.contains(v)).collect()
    }

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let index = |a| nums2.iter().position(|x| *x==a).unwrap()+1;
        for i in nums1{
            let mut left = index(i);
            let mut is_available = false;
            while left < nums2.len(){
                if nums2[left] > i {
                        res.push(nums2[left]);
                        is_available = true;
                        break;
                    }
                    left+=1;
                }
              if !is_available{
                res.push(-1);
              }             
            }
        res
    }

pub fn find_words(words: Vec<String>) -> Vec<String> {
     let row1 = "qwertyuiop".to_string();
     let row2 = "asdfghjkl".to_string();
     let row3 = "zxcvbnm".to_string();
     let first_letter_row = |c: char| {
        let c_lower = c.to_ascii_lowercase();
        if row1.contains(c_lower) {
            row1.clone()
        } else if row2.contains(c_lower) {
            row2.clone()
        } else {
            row3.clone()
        }
     };
     let mut res = vec![];
        for w in words {
            let mut word = String::new();
            let actual_row = first_letter_row(w.chars().nth(0).unwrap());
            for c in w.chars() {
                if actual_row.contains(c.to_ascii_lowercase()) {
                    word.push(c);
                }
            }
            if w == word {
                res.push(word);
            }
        }
        res
    }

pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut candies = HashSet::new();
        let max_candies = candy_type.len()/2;
        for i in candy_type  {
            candies.insert(i);
        }
        if candies.len() == max_candies{
            return max_candies as i32;
        }else if candies.len()<max_candies {
            return candies.len() as i32;
        }else {
            return max_candies as i32;
        }
    }

pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut hash_map = HashMap::new();
        let mut nums = nums;
        nums.sort();
        for i in nums{
            *hash_map.entry(i).or_insert(0)+=1;
        }
        let mut result = vec![];
        for key in hash_map.keys() {
            match (hash_map.get(key),hash_map.get(&(key+1))) {
                (Some(val1),Some(val2)) =>{
                    result.push(*val1+*val2);
                },
                _ =>{},
            }
        }
        let max_val = result.iter().max();
        if max_val.is_some(){
            return *max_val.unwrap();
        }else {
            return 0;
        }
    }