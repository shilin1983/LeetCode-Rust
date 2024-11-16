struct Case {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    expected: f64,
}

#[test]
fn test_find_median_sorted_arrays() {
    use super::Solution;

    let cases: [Case; 2] = [
        Case {
            nums1: vec![1, 3],
            nums2: vec![2],
            expected: 2.0,
        },
        Case {
            nums1: vec![1, 2],
            nums2: vec![3, 4],
            expected: 2.5,
        },
    ];

    for c in cases {
        assert_eq!(
            c.expected,
            Solution::find_median_sorted_arrays(c.nums1, c.nums2),
        )
    }
}
