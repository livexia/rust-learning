/**
找到给定字符串（由小写字符组成）中的最长子串 T ， 要求 T 中的每一字符出现次数都不少于 k 。输出 T 的长度。

示例 1:
输入:
s = "aaabb", k = 3
输出:
3
最长子串为 "aaa" ，其中 'a' 重复了 3 次。

示例 2:
输入:
s = "ababbc", k = 2
输出:
5
最长子串为 "ababb" ，其中 'a' 重复了 2 次， 'b' 重复了 3 次。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-substring-with-at-least-k-repeating-characters
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn longest_substring(s: String, k: i32) -> i32 {
    let n = s.len();
    let s = &s.bytes().map(|b| b as usize - 'a' as usize).collect::<Vec<usize>>();
    dfs(s, 0,n, k as usize)
}

fn dfs(s: &[usize], l: usize, r: usize, k: usize) -> i32 {
    let mut count = vec![0 as usize; 26];
    for i in l..r {
        count[s[i]] += 1;
    }
    
    let mut split = 26;
    for i in 0..26 {
        if count[i] > 0 && count[i] < k {
            split = i;
            break;
        }
    }
    if split == 26 {
        return (r - l) as i32;
    }

    let mut i = l;
    let mut ret = 0;
    while i < r {
        while  i < r && s[i] == split {
            i += 1;
        }
        if i > r - 1 {
            break;
        }
        let start = i;
        while i < r && s[i] != split {
            i += 1;
        }
        let length = dfs(s, start, i, k);
        ret = ret.max(length);
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::longest_substring;

    #[test]
    fn f1() {
        assert_eq!(longest_substring("aaabb".to_string(), 3), 3);
    }

    #[test]
    fn f2() {
        assert_eq!(longest_substring("a".to_string(), 2), 0);
    }
}
