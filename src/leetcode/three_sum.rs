use std::collections::HashSet;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = HashSet::new();
    if nums.len() < 3 {
        return result.into_iter().collect::<Vec<Vec<i32>>>();
    }

    nums.sort();

    for _val in nums.clone() {
        for (i, &val) in nums.iter().enumerate() {
            if i > 0 && val == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let sum = val + nums[left] + nums[right];
                if sum == 0 {
                    result.insert(vec![val, nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
    }
    result.into_iter().collect::<Vec<Vec<i32>>>()
}
