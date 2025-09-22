pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left = 0;
        for i in 0..nums.len() {
            if left == sum - left - nums[i] {
                return i as _;
            }
            left += nums[i]
        }
        -1
    }

pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut x =1;
        let mut res = add_vector(nums.clone(), x);
        
        while res < 1{
            match res {
            n if n < 1 => {
                x+=1;
                res = add_vector(nums.clone(), x)},
            _ => return x
            }
        }
        x
    }

    fn add_vector(nums: Vec<i32>, x:i32) -> i32{
        let mut sum = x;
        let mut min_sum = sum;
        for num in nums {
            sum += num;
            min_sum = min_sum.min(sum);
        }
        min_sum
    }

pub fn max_score(s: String) -> i32 {
        let vec_string = s.chars().map(|x| x).collect::<Vec<char>>();
        let mut total = vec![];
        for x in 1..vec_string.len()  {
            total.push(counting_vec(&vec_string, x));
        }
        *total.iter().max().unwrap() as i32
    }

    fn counting_vec(input:&Vec<char>, index:usize) -> usize{
        let (left, right) = input.split_at(index as usize);
        let sum = left.iter().filter(|x| **x=='0').count() + right.iter().filter(|x| **x=='1').count();
        sum
    }

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res =vec![];
        for i in 1..=len{
            let (arr,_) = nums.split_at(i);
            res.push(arr.iter().sum());
        }
        res
    }

pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..arr.len(){
            for j in i..arr.len(){
                let length = j - i + 1;
                if length%2 != 0{
                    let mut sub_total = 0;
                    for k in i..=j{
                        sub_total+=arr[k];
                    }
                    res+=sub_total;
                }
            }
        }
        res
    }

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut res = Vec::with_capacity(gain.len()+1);
        res.push(0);
        for i in 0..gain.len(){
            res.push(res[i]+gain[i]);
        }
        println!("{res:?}");
        *res.iter().max().unwrap()
    }

pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut arr =vec![0; 101];
        for log in logs.iter(){
            let start = log[0];
            let end = log[1];
            for year in start..end{
                arr[(year-1950) as usize] +=1;
            }

        }
        let mut max_i  = 0;
        let mut max = arr[0];

        for (i, &population) in arr.iter().enumerate(){
            if population > max{
                max_i = i;
                max = population; 
            }
        }
        return (max_i + 1950) as i32
    }

pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    //   let total_vec = ranges.iter().flatten().collect::<Vec<&i32>>(); 
    //   println!("{total_vec:?}");
    //   for x in left..=right{
    //     if !total_vec.contains(&&x){
    //         return false;
    //     }
    //   } 
    //   true
    (left..=right).all(|num| ranges.iter().any(|v| (v[0]..=v[1]).contains(&num)))
    }

pub fn left_right_difference(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for x in 0..nums.len(){
            let actual_value = nums[x];
            nums[x] = 0;
            let (left,right) = nums.split_at(x);
            let value = left.iter().sum::<i32>() - right.iter().sum::<i32>();
            res.push(value.abs());
            nums[x] = actual_value;
        }
        res
    }

pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut value = 0;
        let mut count = 0;
        for x in nums{
            value+=x;
            if value == 0{
                count +=1;
            }
        }
        count
    }

pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let n = nums.len();
    let mut prefix = vec![0; n + 1];

    // build prefix sum
    for i in 0..n {
        prefix[i + 1] = prefix[i] + nums[i];
    }

    let mut result: Option<i32> = None;

    for size in l..=r {
        if size as usize > n {
            continue;
        }
        for start in 0..=n - size as usize {
            let sum = prefix[start + size as usize] - prefix[start];
            if sum > 0 {
                result = Some(match result {
                    Some(min_val) => min_val.min(sum),
                    None => sum,
                });
            }
        }
    }

    result.unwrap_or(-1)
    }

pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut subarray_sum_vec = vec![];
        for i in 0..nums.len(){
            let start: i32 = 0.max(i as i32-nums[i]);
            let sum = (start..=i as i32).collect::<Vec<i32>>();
            let mut val =0;
            for x in sum {
                val+=nums[x as usize];
            }
            subarray_sum_vec.push(val);
        }
        subarray_sum_vec.iter().sum::<i32>()
    }

pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut partition_count = 0;
        for x in 1..nums.len(){
            let (left,right) = nums.split_at(x);
            if (left.iter().sum::<i32>() - right.iter().sum::<i32>())%2 == 0{
                partition_count+=1;
            }
        }
        partition_count
    }

// Medium problems
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut right, mut sum, n) = (0, 0, 0, nums.len());
        let mut ans = n + 1;
        while right < n {
            while right < n && sum < target {
                sum += nums[right];
                right += 1;
            }
            while sum >= target {
                ans = usize::min(ans, right - left);
                sum -= nums[left];
                left += 1;
            }
        }
        if ans == n + 1 { ans = 0; }

        ans as i32
    } 