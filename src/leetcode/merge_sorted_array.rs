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
            if m > 0 { m_index -= 1; }
        } else {
            nums1[write_index] = nums2[n_index];
            n -= 1;
            if n > 0 { n_index -= 1; }
        }
        write_index -= 1;
    }
    
    // If there are remaining elements in nums2, copy them
    while n > 0 {
        nums1[write_index] = nums2[n_index];
        write_index -= 1;
        n -= 1;
        if n > 0 { n_index -= 1; }
    }
    println!("{nums1:?}");
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    return nums.len() as i32;

}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut res = 0;
    for (i,j) in nums.iter().enumerate(){
        if *j == target {
          res = i;
          break;
        }else if *j > target {
            res = i;
            break;
        }else {
            res = i+1;
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