use std::collections::{HashMap, HashSet};

pub fn merge(mut nums1: Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
    let (mut m, mut n) = (m as usize, n as usize);
    let mut write_index = m + n - 1;

    // Convert to 0-indexed
    let mut m_index = if m > 0 { m - 1 } else { 0 };
    let mut n_index = if n > 0 { n - 1 } else { 0 };

    // Work backwards from the end
    while m > 0 && n > 0 {
        if nums1[m_index] > nums2[n_index] {
            nums1[write_index] = nums1[m_index];
            m -= 1;
            if m > 0 {
                m_index -= 1;
            }
        } else {
            nums1[write_index] = nums2[n_index];
            n -= 1;
            if n > 0 {
                n_index -= 1;
            }
        }
        write_index -= 1;
    }

    // If there are remaining elements in nums2, copy them
    while n > 0 {
        nums1[write_index] = nums2[n_index];
        write_index -= 1;
        n -= 1;
        if n > 0 {
            n_index -= 1;
        }
    }
    println!("{nums1:?}");
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    return nums.len() as i32;
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut res = 0;
    for (i, j) in nums.iter().enumerate() {
        if *j == target {
            res = i;
            break;
        } else if *j > target {
            res = i;
            break;
        } else {
            res = i + 1;
        }
    }
    return res as i32;
}

pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for x in digits.iter_mut().rev() {
        match *x == 9 {
            true => *x = 0,
            false => {
                *x += 1;
                return digits;
            }
        }
    }
    digits.insert(0, 1);
    digits
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] < buy {
            buy = prices[i];
        } else if prices[i] - buy > profit {
            profit = prices[i] - buy;
        }
    }
    profit
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut triangle: Vec<Vec<i32>> = Vec::new();
    if num_rows <= 0 {
        return triangle;
    }
    for i in 0..num_rows as usize {
        let mut row = vec![1; i + 1];
        for j in 1..i {
            row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
        }
        triangle.push(row);
    }
    triangle
}

pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut res = vec![1];
    let mut prev: i64 = 1; // use i64 for the calculations
    for k in 1..=row_index {
        let next_val = prev * (row_index - k + 1) as i64 / k as i64;
        res.push(next_val as i32);
        prev = next_val;
    }
    res
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut value = HashMap::new();
    for i in 0..nums.len() {
        *value.entry(nums[i]).or_insert(0) += 1;
    }
    value
        .iter()
        .find(|(_, &value)| value == 1)
        .map(|(key, _)| *key)
        .unwrap_or(0)
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut res = HashMap::new();
    let n = nums.len() as i32;
    for i in 0..nums.len() {
        *res.entry(nums[i]).or_insert(0) += 1;
    }
    for (x, y) in res {
        if y > n / 2 {
            return x;
        }
    }
    -1
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut res = HashMap::new();
    for i in 0..nums.len() {
        *res.entry(nums[i]).or_insert(0) += 1;
    }
    for (_, y) in res {
        if y > 1 {
            return true;
        }
    }
    false
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut seen = HashSet::with_capacity(k + 1);

    for (i, &num) in nums.iter().enumerate() {
        if !seen.insert(num) {
            return true;
        }
        if i + 1 > k {
            seen.remove(&nums[i - k]);
        }
    }
    false
}
