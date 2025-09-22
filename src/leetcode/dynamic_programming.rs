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