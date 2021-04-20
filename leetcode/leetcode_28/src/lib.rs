/**
实现 strStr() 函数。

给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串出现的第一个位置（下标从 0 开始）。如果不存在，则返回  -1 。

说明：
当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。

对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与 C 语言的 strstr() 以及 Java 的 indexOf() 定义相符。

示例 1：
输入：haystack = "hello", needle = "ll"
输出：2

示例 2：
输入：haystack = "aaaaa", needle = "bba"
输出：-1

示例 3：
输入：haystack = "", needle = ""
输出：0

提示：
0 <= haystack.length, needle.length <= 5 * 104
haystack 和 needle 仅由小写英文字符组成


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/implement-strstr
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() { return 0 }
    let b1  = haystack.as_bytes();
    let b2 = needle.as_bytes();
    let n = b1.len();
    let m = b2.len();
    let mut pi = vec![0; m];

    let mut j = 0;
    for i in 1..m {
        while j > 0 && b2[i] != b2[j] {
            j = pi[j - 1];
        }
        if b2[i] == b2[j] {
            j += 1;
        }
        pi[i] = j;
    }

    let mut j = 0;
    for i in 0..n {
        while j > 0 && b1[i] != b2[j] {
            j = pi[j - 1];
        }
        if b1[i] == b2[j] {
            j += 1;
        }
        if j == m {
            return  (i - m + 1) as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::str_str;

    #[test]
    fn it_works() {
        assert_eq!(str_str("mississippi".to_string(), "issip".to_string()), 4);
    }
}
