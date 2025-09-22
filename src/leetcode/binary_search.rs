pub struct Solution{
    pub bad_version: i32
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
		let mut left = 1;
        let mut right = n;
        while left<right {
            let mid = left + (right-left)/2;
            if self.is_bad_version(mid){
                right = mid;
            }else {
                left = mid+1;
            }
        }
        left
    }
    fn is_bad_version(&self, version:i32) -> bool {
        if version == self.bad_version{
            return true;
        }else {
            return false;
        }
    }

    pub unsafe fn _guess_number(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left<right{
            let mid = left + (right-left)/2;
            let guessed_value = mock_guess(mid);
            if guessed_value == 1{
                left = mid+1;
            }else {
                right = mid;
            }
        }
        left
    }
}

#[allow(dead_code)]
fn mock_guess(num: i32) -> i32 {
    let pick = 2;  // You can change this value
    if num > pick {
        -1
    } else if num < pick {
        1
    } else {
        0
    }
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if !nums.contains(&target){
            return -1;
        }
        let mut left = 0;
        let mut right = nums.len()-1;
        let mut res =0;
        while left<=right{
            let mid = left + (right-left)/2;
            let value = nums[mid as usize];
            if value == target{
                res = mid;
                break;
            }else if value < target{
                left = mid+1;
            }else {
                right = mid;
            }
        }
        res as i32
    }

pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let byte_character = letters.iter().map(|x| *x as u8).collect::<Vec<_>>();
        let target_u8 = target as u8;
        let binding = byte_character.into_iter().find(|x| *x > target_u8);
        let val = binding.iter().collect::<Vec<_>>();
        if binding.is_some() {
            return *val[0] as char;
        }else {
            return letters[0];
        }
    }

pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
       let mut a = mat
            .iter()
            .map(|xs| xs.iter().filter(|&&x| x == 1).count())
            .enumerate()
            .collect::<Vec<_>>();
        a.sort_by_key(|&x| x.1);
        (0..k as usize).map(|i| a[i].0 as i32).collect()
    }

pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let base_0 = arr.iter().filter(|&&x| x == 0).count();
        if base_0 >= 2{
            return true;
        }
        let mut index = 0;
        let mut res = false;
        while index<arr.len() {
            let base = arr[index];
            if arr.contains(&(2 * base)) && base != 0{
                res = true;
                break;
            }else {
                index+=1;
            }
        }
        res
    }

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let negative_count: i32 = grid.iter().map(|x| x.iter().filter(|&y| y.is_negative()).count() as i32).sum();
        negative_count
    }

pub fn find_the_distance_value(arr1: Vec<i32>,mut arr2: Vec<i32>, d: i32) -> i32 {
        arr2.sort_unstable();
        arr1.iter().fold(0, |res, &x| {
            res + (arr2.iter().all(|&y| (x - y).abs() > d) as i32)
        })
    }

pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let max = arr.last().copied().unwrap_or(0)+k;
        let mut temp = vec![];
        for i in 1..=max{
            temp.push(i);
        }
        for x in arr {
            if temp.contains(&x){
                let index =  temp.iter().position(|y| *y ==x).unwrap();
                temp.remove(index);
            }
        }
        temp[k as usize-1]
       }

pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut right = 1;
        let mut left = nums.len() as i32;

        while right <= left {
            let half = ((left - right) / 2) + right;

            // check half
            let mut count = 0;
            for &n in &nums {
                if n >= half {
                    count += 1;
                }
            }
            if count == half {
                return half;
            } else if count > half {
                right = half + 1;
            } else {
                left = half - 1;
            }
        }

        -1
    }

pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut index = 0;
        let mut res = vec![];
        for x in nums{
            if x == target{
                res.push(index);
            }
            index+=1;
        }
        res
    }

pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut nums = nums;
        nums.sort();
        for x in queries{
            let mut sub_sequence = vec![];
            let mut sum = 0;
            for y in &nums{
                sum = sum+y;
                if x >= sum{
                    sub_sequence.push(y);
                }else {
                    break;
                }
            }
            res.push(sub_sequence.len() as i32);
        }
        res
    }