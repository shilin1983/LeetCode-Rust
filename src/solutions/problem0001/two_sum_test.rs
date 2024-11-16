struct Case {
    nums: Vec<i32>,
    target: i32,
    expected: Vec<i32>,
}

#[test]
fn test_two_sum() {
    use super::Solution;

    let cases: [Case; 3] = [
        Case {
            nums: vec![2, 7, 11, 15],
            target: 9,
            expected: vec![0, 1],
        },
        Case {
            nums: vec![3, 2, 4],
            target: 6,
            expected: vec![1, 2],
        },
        Case {
            nums: vec![3, 3],
            target: 6,
            expected: vec![0, 1],
        },
    ];

    for c in cases {
        assert_eq!(c.expected, Solution::two_sum(c.nums, c.target))
    }
}
