/**
输入一个字符串，打印出该字符串中字符的所有排列。

你可以以任意顺序返回这个字符串数组，但里面不能有重复元素。

示例:

输入：s = "abc"
输出：["abc","acb","bac","bca","cab","cba"]
 
限制：
1 <= s 的长度 <= 8

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/zi-fu-chuan-de-pai-lie-lcof
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn permutation(s: String) -> Vec<String> {
    let n = s.len();
    let mut res = vec![];
    let mut t = String::with_capacity(n);
    let mut visited = vec![false; n];
    let mut s = s.into_bytes();
    s.sort();
    fn backtrace(pos: usize, res: &mut Vec<String>, t: &mut String, visited: &mut Vec<bool>, n: usize, s: &[u8]) {
        if pos == n {
            res.push(t.clone());
            return;
        }
        for i in 0..n {
            if visited[i] || (i > 0 && s[i - 1] == s[i] && visited[i - 1]) {
                continue;
            }
            t.push(s[i] as char);
            visited[i] = true;
            backtrace(pos + 1, res, t, visited, n, s);
            t.pop();
            visited[i] = false;
        }
    }
    backtrace(0, &mut res, &mut t, &mut visited, n, &s[..]);
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
