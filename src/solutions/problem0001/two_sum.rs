use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_table = HashMap::<i32, i32>::with_capacity(nums.len());

        for (key, &value) in nums.iter().enumerate() {
            // * 计算目标值与当前元素的差值
            let diff = target - value;

            // * 如果哈希表中存在差值，则返回差值与当前元素的索引
            if let Some(&idx) = hash_table.get(&diff) {
                return vec![idx, key as i32];
            }

            // * 将当前元素及其索引插入哈希表
            hash_table.insert(value, key as i32);
        }

        vec![]
    }
}
