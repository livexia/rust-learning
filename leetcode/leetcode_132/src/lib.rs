/**
给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是回文。

返回符合要求的 最少分割次数 。

示例 1：
输入：s = "aab"
输出：1
解释：只需一次分割就可将 s 分割成 ["aa","b"] 这样两个回文子串。

示例 2：
输入：s = "a"
输出：0

示例 3：
输入：s = "ab"
输出：1
 
提示：

1 <= s.length <= 2000
s 仅由小写英文字母组成

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/palindrome-partitioning-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
pub fn min_cut(s: String) -> i32 {
    let chars: Vec<u8> = s.bytes().collect();
    let n = s.len();
    let mut dp: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..=i {
            if chars[i] == chars[j] && ( i - j <= 2 || dp[j+1][i-1] ) {
                dp[j][i] = true;
            }
        }
    }
    let mut f = vec![std::i32::MAX; n];
    for i in 0..n {
        if dp[0][i] {
            f[i] = 0;
        } else {
            for j in 0..i {
                if dp[j+1][i] {
                    f[i] = f[i].min(f[j] + 1)
                }
            }
        }
    }

    f[n-1]
}

#[cfg(test)]
mod tests {
    use crate::min_cut;

    #[test]
    fn it_works() {
        assert_eq!(min_cut("aba".to_string()), 0);
    }
}
