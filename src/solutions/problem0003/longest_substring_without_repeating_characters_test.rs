struct Case {
    s: String,
    expected: i32,
}

#[test]
fn test_length_of_longest_substring() {
    use super::Solution;

    let cases: [Case; 3] = [
        Case {
            s: "abcabcbb".to_string(),
            expected: 3,
        },
        Case {
            s: "bbbbb".to_string(),
            expected: 1,
        },
        Case {
            s: "pwwkew".to_string(),
            expected: 3,
        },
    ];

    for c in cases {
        assert_eq!(c.expected, Solution::length_of_longest_substring(c.s))
    }
}
