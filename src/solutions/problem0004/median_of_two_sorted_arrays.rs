pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // * 合并两个数组，并排序
        let mut merged = nums1;
        merged.append(&mut nums2.clone());
        merged.sort();

        let mid = merged.len() / 2;
        if merged.len() % 2 == 0 {
            // * 如果合并后的数组长度为偶数，则中位数为中间两个数的平均值
            (merged[mid - 1] + merged[mid]) as f64 / 2.0
        } else {
            // * 如果合并后的数组长度为奇数，则中位数为中间的数
            merged[mid] as f64
        }
    }
}
