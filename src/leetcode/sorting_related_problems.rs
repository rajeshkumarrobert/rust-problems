use std::collections::HashMap;

pub fn third_max(nums: Vec<i32>) -> i32 {
       let mut nums =nums;
       nums.sort();
       nums.dedup();
       let mut max = nums.last().unwrap();
       if nums.len() < 3{
        return *max;
       }
       let mut temp =0;
       let mut old_value =0;
       for i in nums.iter().rev(){
        if i < max && old_value !=*i{ 
            old_value = *i;
            max =i;
            temp +=1;
        }
        if temp ==2{
            break;
        }
        
       }
       *max
}

pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut g = g;
        let mut s = s;
        g.sort();
        s.sort();
        for &cookie in s.iter() {
            if count < g.len() && cookie >= g[count] {
                count += 1;
            }
        }
        count as i32
    }

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
       let n = score.len();
        let mut pairs: Vec<(i32, usize)> = score.into_iter().zip(0..n).collect();
        pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut ans = vec![String::new(); n];
        let medals = ["Gold Medal", "Silver Medal", "Bronze Medal"];
        for (rank, &(_sc, idx)) in pairs.iter().enumerate() {
            ans[idx] = if rank < 3 {
                medals[rank].to_string()
            } else {
                (rank + 1).to_string()
            };
        }
        ans
    }

pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut tuple_vec = vec![];
        let mut n = 0;
        while n<nums.len() {
            tuple_vec.push((nums[n],nums[n+1]));
            n+=2;
        }
        let res:Vec<i32> = tuple_vec.iter().map(|x| x.0.min(x.1)).collect();
        res.iter().sum()
    }

pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.reverse();
        let max = nums[0] * nums[1] * nums[2];
        let min = nums[0] * nums[nums.len()-2] * nums[nums.len()-1];
        max.max(min)
    }

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let total_len = nums.len();
        let mut freq = HashMap::new();
        for i in nums{
            *freq.entry(i).or_insert(0)+=1;
        }
        let expected_vec:Vec<i32> = [1..=total_len].into_iter().flat_map(|range|range.map(|x: usize| x as i32)).collect();
        let expected_total:i32 = expected_vec.iter().sum();
        let mut repeated_value = 0;
        for (key,value) in freq.clone(){
            if value ==2{
                repeated_value = key;
                break;
            }
        }
        let missing_value:i32 = expected_total - (freq.keys().sum::<i32>());
        vec![repeated_value,missing_value]
    }

pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let max_val = *nums.iter().max().unwrap();
        let mut res = true;
        for i in nums.clone()  {
            if i != max_val{
                if 2*i > max_val{
                    res = false;
                    break;
                }
            }  
        }
        if res {
            return nums.iter().position(|x| *x==max_val).unwrap() as i32;
        }else {
            return -1;
        }
    }

pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let bob_total:i32 = bob_sizes.iter().sum();
        let alice_total:i32 = alice_sizes.iter().sum();
        let diff = (bob_total-alice_total)/2;
        let mut result = vec![];
        for x in alice_sizes{
            let y = x+diff;
            if bob_sizes.contains(&y){
                result.push(x);
                result.push(y);
                break;
            }
        }
        result
    }

pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for x in nums.clone(){
            if x%2 ==0{
                res.push(x);
            }
        }
        for x in nums{
            if x%2!=0{
                res.push(x);
            }
        }
        res
    }

pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res = vec![0; len];
        let mut n = 0;
        for x in nums.iter() {
            if x % 2 == 0 {
                res[2 * n] = *x;
                n += 1;
            }
        }
        let mut m = 0;
        for x in nums.iter() {
            if x % 2 != 0 {
                res[2 * m + 1] = *x;
                m += 1;
            }
        }
        res
    }

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let (mut left, mut right, mut pos) = (0, nums.len() - 1, nums.len() - 1);
        while left <= right {
            let left_sq = nums[left] * nums[left];
            let right_sq = nums[right] * nums[right];
            if left_sq > right_sq {
                res[pos] = left_sq;
                left += 1;
            } else {
                res[pos] = right_sq;
                if right == 0 { break; }
                right -= 1;
            }
            if pos == 0 { break; }
            pos -= 1;
        }
        res
    }

