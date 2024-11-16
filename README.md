# leetcode-rust

- LeetCode 题解项目，使用 [Rust](https://www.rust-lang.org/) 语言实现。

- 帮助开发者更好地了解和学习数据结构与算法。

## 获取代码

```git
git clone https://github.com/shilin1983/leetcode-rust.git
```

## 安装依赖

```bash
cargo install cargo-llvm-cov
```

```bash
rustup component add llvm-tools-preview
```

## 运行测试

```shell
sh test.sh
```

## 题解列表

| 序号  |                                                 标题                                                 | 难度  |                                        方案                                         |
| :---: | :--------------------------------------------------------------------------------------------------: | :---: | :---------------------------------------------------------------------------------: |
| 0001  |                          [两数之和](https://leetcode.cn/problems/two-sum/)                           | 简单  |                    [Rust](src/solutions/problem0001/two_sum.rs)                     |
| 0002  |                      [两数相加](https://leetcode.cn/problems/add-two-numbers/)                       | 中等  |                [Rust](src/solutions/problem0002/add_two_numbers.rs)                 |
| 0003  | [无重复字符的最长子串](https://leetcode.cn/problems/longest-substring-without-repeating-characters/) | 中等  | [Rust](src/solutions/problem0003/longest_substring_without_repeating_characters.rs) |
