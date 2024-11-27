use std::collections::HashMap;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut last_index = 0;
    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;
    
    while left < right {
        if nums[left] < target && target < nums[right] {
            left += 1;
            right -= 1;
            continue;
        }
        if target == nums[right] {
            return right as i32;
        }
        if target == nums[left] {
            return left as i32;
        }
        if target > nums[right] {
            return right as i32 + 1;
        }
        if target < nums[left] {
            return left as i32 ;
        }
        if target > nums[left] && target < nums[right] {
            return left as i32 + 1;
        }
        if target > nums[left] && target < nums[right] {
            return left as i32 + 1;
        }
    }
    if target == nums[right] {
        return right as i32;
    }
    if target == nums[left] {
        return left as i32;
    }
    if target > nums[right] {
        return right as i32 + 1;
    }
    if target < nums[left] {
        return left as i32 ;
    }
    if target > nums[left] && target < nums[right] {
        return left as i32 + 1;
    }
    if target > nums[left] && target < nums[right] {
        return left as i32 + 1;
    }

    // 找不到的情况

    0
}
