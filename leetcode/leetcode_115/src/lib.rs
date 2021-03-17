/**
给定一个字符串 s 和一个字符串 t ，计算在 s 的子序列中 t 出现的个数。

字符串的一个 子序列 是指，通过删除一些（也可以不删除）字符且不干扰剩余字符相对位置所组成的新字符串。（例如，"ACE" 是 "ABCDE" 的一个子序列，而 "AEC" 不是）

题目数据保证答案符合 32 位带符号整数范围。

示例 1：
输入：s = "rabbbit", t = "rabbit"
输出：3
解释：
如下图所示, 有 3 种可以从 s 中得到 "rabbit" 的方案。
(上箭头符号 ^ 表示选取的字母)
rabbbit
^^^^ ^^
rabbbit
^^ ^^^^
rabbbit
^^^ ^^^

示例 2：
输入：s = "babgbag", t = "bag"
输出：5
解释：
如下图所示, 有 5 种可以从 s 中得到 "bag" 的方案。 
(上箭头符号 ^ 表示选取的字母)
babgbag
^^ ^
babgbag
^^    ^
babgbag
^    ^^
babgbag
  ^  ^^
babgbag
    ^^^

提示：

0 <= s.length, t.length <= 1000
s 和 t 由英文字母组成

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/distinct-subsequences
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn num_distinct(s: String, t: String) -> i32 {
    let m = s.len();
    let n = t.len();
    if m < n { return 0; }
    let mut dp = vec![vec![0; n + 1];  m + 1];
    for i in 0..=m {
        dp[i][n] = 1;
    }
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    for i in (0..m).rev() {
        let s_c = s_chars[i];
        for j in (0..n).rev() {
            let t_c = t_chars[j];
            if s_c == t_c {
                dp[i][j] = dp[i+1][j+1] + dp[i+1][j];
            } else {
                dp[i][j] = dp[i+1][j]
            }
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use crate::num_distinct;

    #[test]
    fn it_works() {
        assert_eq!(num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }
}
