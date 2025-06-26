use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let diff = target - nums[i];
            match map.get(&diff) {
                Some(j) => return vec![*j, i as i32],
                None => map.insert(nums[i], i as i32),
            };
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
    
    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
    }
    
    #[test]
    fn test_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}