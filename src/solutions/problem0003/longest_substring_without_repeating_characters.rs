use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let length = s.len();
        let (mut left, mut right, mut max_length, mut hash_table) =
            (0, 0, 0, HashMap::<char, i32>::with_capacity(length));

        while right < length {
            let char = s.chars().nth(right).unwrap();

            // * 如果哈希表中存在当前元素，则移动滑动窗口左边界到当前元素的下一个位置
            if let Some(&idx) = hash_table.get(&char) {
                left = left.max(idx + 1);
            }

            // * 将当前元素及其索引插入哈希表
            hash_table.insert(char, right as i32);
            // * 移动滑动窗口右边界
            right += 1;
            // * 计算当前滑动窗口最大长度
            max_length = max_length.max(right as i32 - left);
        }

        max_length
    }
}
