use std::i32;
pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..=n{
            let binary_digits:Vec<_> = format!("{:b}", i).chars()
                                            .map(|c| c.to_digit(2).unwrap()).collect();
            let value = binary_digits.iter().filter(|x| **x == 1).count();
            res.push(value as i32);
        }
        res
    }

pub fn fib(n: i32) -> i32 {
    if n <= 1 {
		return n;
	}

	return fib(n - 1) + fib(n - 2);
    }

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let (mut a, mut b) = (cost[0], cost[1]);
        for i in 2..n {
            let c = cost[i] + a.min(b);
            a = b;
            b = c;
        }
        a.min(b)
    }

pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }

pub fn tribonacci(n: i32) -> i32 {
        let mut vec_value = [0;41];
        vec_value[0] = 0;
        vec_value[1] = 1;
        vec_value[2] = 1;

        for x in 0..=n as usize{
            vec_value[x+3] = vec_value[x]+vec_value[x+1]+vec_value[x+2];
        }

        vec_value[n as usize]
    }

pub fn max_repeating(sequence: String, word: String) -> i32 {
    let n = sequence.len();
    let m = word.len();
    let s_bytes = sequence.as_bytes();
    let w_bytes = word.as_bytes();

    let mut dp = vec![0; n + 1];
    let mut result = 0;

    for i in m..=n {
        if &s_bytes[i - m..i] == w_bytes {
            dp[i] = dp[i - m] + 1;
            result = result.max(dp[i]);
        }
    }

    result as i32
    }

pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        res.push(words[0].clone());
        for i in 1..groups.len(){
            if groups[i]!=groups[i-1]{
                res.push(words[i].clone());
            }
        }
        res
    }

//medium problems
pub fn longest_palindrome(s: String) -> String {
        if s.is_empty(){
            return "".to_string();
        }

        let mut altered_string = String::from("^");
        for c in s.chars(){
            altered_string.push('#');
            altered_string.push(c);
        }
        altered_string.push_str("#$");

        let n = altered_string.len();
        let mut p = vec![0;n];
        let mut center = 0;
        let mut right = 0;
        
        let chars:Vec<char> = altered_string.chars().collect();

        for i in 1..n-1{
            
            if i < right{
                let mirror = 2*center - i;
                p[i] = (right - i).min(p[mirror]);
            }

            while chars[i + 1 + p[i]] == chars[i - 1 - p[i]]{
                p[i]+=1;
            }

            if i+p[i] > right{
                center = i;
                right = i + p[i];
            }
        }

        let (max_radius, max_idx) = p
            .iter()
            .enumerate()
            .max_by_key(|(_, &radius)| radius)
            .map(|(idx, &radius)| (radius, idx))
            .unwrap_or((0, 0));

        
        let mut longest_palindromic_substring = String::new();
        
        for i in (max_idx - max_radius)..=(max_idx + max_radius) {
            if i < altered_string.len() && chars[i] != '#' {
                longest_palindromic_substring.push(chars[i]);
            }
        }
        
        longest_palindromic_substring
    }

pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(result:&mut Vec<String>, current:String, open:i32 ,close:i32 , max:i32){
            if current.len() as i32 == 2 * max{
                result.push(current);
                return;
            }

            if open < max{
                backtrack(result, format!("{}(",current), open+1, close, max);
            }
            if close < open{
                backtrack(result, format!("{})",current), open, close+1, max);
            }
        }

        let mut result = Vec::new();
        backtrack(&mut result, String::new(), 0, 0, n);
        result
    }

pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut current_end = 0;
        let mut farthest = 0;

        for i in 0..nums.len() - 1 {
            farthest = farthest.max(i + nums[i] as usize);

            if i == current_end {
                jumps += 1;
                current_end = farthest;
            }
        }

        jumps
    }

pub fn max_sub_array(nums: Vec<i32>) -> i32{
    let mut current_sum = 0;
    let mut max_sum = i32::MIN;
    let mut min_value = i32::MIN;

    for num in nums{
        current_sum += num;
        max_sum = max_sum.max(current_sum);
        if current_sum < 0{
            current_sum = 0;
        }

        min_value = min_value.max(num);
    }

    if max_sum == 0{
        return min_value;
    }else {
        return max_sum;
    }
} 

pub fn can_jump(nums: Vec<i32>) -> bool {
        let jumps = nums.len()-1;
        let mut current_end = 0;
        let mut farthest = 0;

        for i in 0..nums.len() {
            farthest = farthest.max(i + nums[i] as usize);
            if i <= current_end {
                current_end = farthest;
                if i == jumps{
                    return true;
                }
            }
        }

        false
    }

pub fn unique_paths(m: i32, n: i32) -> i32 {
        //(m+n-2)!/((m-1)!(n-1))!
        let m1 = m.max(n);
        let m2 = m.min(n);
        let fact1 : u128 = (m1..m+n-1).fold(1,|acc,x| acc * (x as u128));
        let fact2 : u128 = (1..m2).fold(1,|acc,x| acc * (x as u128));
        return (fact1/fact2) as i32;
    }

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() || obstacle_grid[0].is_empty() || obstacle_grid[0][0] == 1 {
            return 0;
        }
        let _m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        
        let mut previous = vec![0; n];
        let mut current = vec![0; n];
        previous[0] = 1;
        
        for row in &obstacle_grid {
            current[0] = if row[0] == 1 { 0 } else { previous[0] };
            for j in 1..n {
                if row[j] == 1 {
                    current[j] = 0;
                } else {
                    current[j] = current[j-1] + previous[j];
                }
            }
            std::mem::swap(&mut previous, &mut current);
        }
        
        previous[n-1]
    }

pub fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    // Convert to char vectors for easy indexing
    let word1_chars: Vec<char> = word1.chars().collect();
    let word2_chars: Vec<char> = word2.chars().collect();
    // DP table of size (m+1) x (n+1)
    let mut dp = vec![vec![0; n + 1]; m + 1];
    // Base cases
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    // Fill DP table
    for i in 1..=m {
        for j in 1..=n {
            if word1_chars[i - 1] == word2_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j - 1], // substitution
                    std::cmp::min(
                        dp[i - 1][j],   // deletion
                        dp[i][j - 1],   // insertion
                    ),
                );
            }
        }
    }
    dp[m][n] as i32
    }