pub fn is_palindrome(x: i32) -> bool {
       let string_i32 = x.to_string().chars().collect::<Vec<char>>();
       let mut left = 0;
       let mut right = string_i32.len()-1;
        while left<right {
            if string_i32[left] != string_i32[right]{
                return false;
            }
            left+=1;
            right-=1;
        }
        return true;
    }

 pub fn my_sqrt(x: i32) -> i32 {
        let mut low = 0;
        let mut high = std::cmp::min(x, 46340);

        while low <= high {
            let mid = low + (high-low)/2;
            if mid * mid == x {
                return mid;
            }
            else if mid * mid > x {
                high = mid - 1;
            }
            else {
                low = mid + 1;
            }
        }
        high
    }

pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
        return n;
    }
    
    let mut a = 1;
    let mut b = 1;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
    }

pub fn is_happy(n: i32) -> bool {
     if n==1 || n==7{
        return true;
     }
     let mut n = n;
     println!("{:?}",n.to_string().len());
     while (n.to_string().len())!=1 {
         n = convert_to_single_digit(n)
     }

     if n ==1 || n==7{
        return true;
     }else {
         return false;
     }
       
    }

    pub fn convert_to_single_digit(mut n:i32) -> i32{
    let mut vec_digits = Vec::new();
     while n > 0 {
         vec_digits.push(n%10);
         n/=10;
     }
     vec_digits.reverse();
     println!("{vec_digits:?}");
     let mut temp = 0;
     for i in vec_digits.clone(){
         temp = temp + (i*i);
     }
     temp
    }

pub fn is_power_of_two(n: i32) -> bool {  
        let mut n = n;
        while n > 0 {
            if n == 1 {
                return true;
            } else if n % 2 == 0 {
                n = n / 2;
            } else {
                return false;
            }
        }
        false
    }

pub fn add_digits(num: i32) -> i32 {
    if num < 10{
        return num;
    }
    let mut num = num;
    let mut temp = 0;
        while num > 0{
            temp += num%10;
            num/=10;
        }
    add_digits(temp)
    }

pub fn is_ugly(mut n: i32) -> bool {
    if n == 0 {
        return false;
    }
        for i in [2, 3, 5]{
            while n % i == 0{
                n /= i;
            }
        }
        n == 1

    }

pub fn can_win_nim(n: i32) -> bool {
        if n%4 !=0{
            return true;
        }else{
            return false;
        }
    }

pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            if n == 1 {
                return true;
            } else if n % 3 == 0 {
                n = n / 3;
            } else {
                return false;
            }
        }
        false
    }

pub fn is_power_of_four(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            if n == 1 {
                return true;
            } else if n % 4 == 0 {
                n = n / 4;
            } else {
                return false;
            }
        }
        false
    }

pub fn is_perfect_square(num: i32) -> bool {
        let mut low = 0;
        let mut high = std::cmp::min(num, 46340);
        let mut res = false;
        while low <= high {
            let mid = low + (high-low)/2;
            if mid * mid == num {
                res = true;
                break;
            }
            else if mid * mid > num {
                high = mid - 1;
            }
            else {
                low = mid + 1;
            }
        }
        res
    }

pub fn add_strings(num1: String, num2: String) -> String {
      let number = num1.parse::<i128>().unwrap()+num2.parse::<i128>().unwrap();
      number.to_string()
    }

pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let (mut l, mut r) = (0, n);
        while l < r {
            let m = l + (r - l) / 2;
            if n >= (m + 1) * (m + 2) / 2 {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l as i32
    }

pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let area_f64 = area as f64; 
        let mut w = area_f64.sqrt();
        let mut l ;
        loop {
            l = area/w as i32;
            if l*w as i32 == area{
                break;
            }
            w = (w as i32 - 1) as f64;
        }
        vec![l,w as i32]
    }

pub fn check_perfect_number(num: i32) -> bool {

        num == sum_slice(&get_divisors(num))
    }

pub fn get_divisors(num: i32) -> Vec<i32> {
        if num <= 1 {
            return vec![];
        }
        let mut result = vec![1];
        let limit = (num as f64).sqrt() as i32;
        for i in 2..=limit {
            if num % i == 0 {
                result.push(i);
                let pair = num / i;
                if pair != i {
                    result.push(pair);
                }
            }
        }
        result
    }

pub fn sum_slice(nums: &[i32]) -> i32 {
        nums.iter().sum()
    }