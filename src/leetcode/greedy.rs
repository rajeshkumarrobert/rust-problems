pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut n = n;
    let mut flowerbed = flowerbed;
    let len = flowerbed.len();
    for i in 1..len {
        match (flowerbed.get(i - 1), flowerbed.get(i), flowerbed.get(i + 1)) {
            (Some(0), Some(0), Some(0))
            | (None, Some(0), Some(0))
            | (None, Some(0), None)
            | (Some(0), Some(0), None) => {
                n -= 1;
                flowerbed[i] = 1;
            }
            _ => {}
        }
    }
    n <= 0
}

pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut bill5 = 0;
        let mut bill10 = 0;
        let mut res:bool = true;
        for x in bills{
            match x {
                5 => bill5+=1,
                10 => {
                    if bill5>=1{
                        bill10+=1;
                        bill5-=1;
                    }else {
                        res = false;
                        break;
                    }
                },
                20 => 
                {
                    if bill10!=0 && bill5 >= 1{
                        bill10-=1;
                        bill5-=1;
                    }else if bill5 >= 3{
                        bill5-=3;
                    }else {
                        res = false;
                        break;
                    }
                },
                _ =>{},
            }
        }
        res
    }

pub fn di_string_match(s: String) -> Vec<i32> {
        let mut res = vec![];
        let mut d = s.len() as i32;
        let mut i = 0;
        let mut s = s;
        s.push('D');
        for x in s.chars() {
            match x {
                'D' => {
                    res.push(d);
                    d-=1;
                },
                'I' => {
                    res.push(i);
                    i+=1;
                },
                _ =>{}
            }         
        }
        res
    }

pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        
        for i in (2..n).rev() {
            if nums[i-2] + nums[i-1] > nums[i] {
                return nums[i-2] + nums[i-1] + nums[i];
            }
        }
        
        0
    }

pub fn largest_sum_after_k_negations(mut nums: Vec<i32>,mut k: i32) -> i32 {
        nums.sort();
        for i in 0..nums.len(){
            if k != 0 {
                if nums[i].is_negative(){
                    nums[i] = -nums[i];
                    k-=1;
                }
            }else {
                break;
            }
        }

        if k>0{
            for i in 0..nums.len() {
                if nums[i] == 0{
                    while k>0 {
                       nums[i] = -nums[i];
                       k-=1; 
                    }
                }
            }
            if k>0{
                nums.sort();
                while k>0 {
                       nums[0] = -nums[0];
                       k-=1; 
                    }
            }
        }
        nums.iter().sum()
    }

pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        if arr.iter().all(|x| *x==0){
            return true;
        }
        let total:i32 = arr.iter().sum();
        let each_partition = if total % 3 == 0 { total / 3 } else { return false; };
        let mut temp = 0;
        let mut partition_count = 3;
        for x in arr{
            temp+=x;
            if temp == each_partition{
                partition_count-=1;
                temp=0;
            }
        }

        partition_count<=0
    }

pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
      let even = position.iter().filter(|x| **x%2==0).count(); 
      let odd = position.iter().filter(|x| **x%2!=0).count();
      even.min(odd) as i32
    }

pub fn balanced_string_split(s: String) -> i32 {
        let mut res = 0;
        let mut balance = 0;
        for c in s.chars(){
            match c {
                'R'=> balance+=1,
                _ => balance-=1
            }
            if balance==0{
                res+=1;
            }
        }
        res
    }

pub fn maximum69_number (num: i32) -> i32 {
        let mut num = num.to_string().chars().collect::<Vec<char>>();
        for i in 0..num.len(){
            if num[i] =='6'{
                num[i] = '9';
                break;
            }
        }
    //num.iter().collect::<String>().parse().unwrap()
    let res = num.into_iter().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<_>>();
    res.iter().fold(0, |acc, &d| acc * 10 + d)
    
}

pub fn max_area(height: Vec<i32>) -> i32 {
        let mut right = height.len()-1;
        let mut left = 0;
        let mut area = 0;
        while left < right{
            let breadth = height[left].min(height[right]);
            let length = (right-left) as i32;
            area = area.max(length*breadth);
            if height[left] < height[right]{
                left+=1;
            }else{   
                right-=1;
            } 
        }
        area
    }

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] < buy {
                buy = prices[i];
            } else if prices[i] - buy > 0 {
                profit += prices[i] - buy;
                buy = prices[i];
            }
        }
    profit
    }

 pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut total_tank = 0;
        let mut curr_tank = 0;
        let mut start = 0;
        for i in 0..n {
            let diff = gas[i] - cost[i];
            total_tank += diff;
            curr_tank += diff;

            if curr_tank < 0 {
                start = i + 1;
                curr_tank = 0;
            }
        }

        if total_tank >= 0 { start as i32 } else { -1 }
    }