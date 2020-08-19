use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            let num2 = target - n;
            if let Some(num) = num_map.get(&num2) {
                return vec![*num as i32, i as i32];
            }
            num_map.insert(n, i);
        }
        Vec::new()
    }
}
