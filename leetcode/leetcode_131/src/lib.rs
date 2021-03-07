/**
给定一个字符串 s，将 s 分割成一些子串，使每个子串都是回文串。

返回 s 所有可能的分割方案。

示例:

输入: "aab"
输出:
[
  ["aa","b"],
  ["a","a","b"]
]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/palindrome-partitioning
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn partition_backtrace(s: String) -> Vec<Vec<String>> {
    let mut res = vec![];
    let chars: Vec<u8> = s.bytes().collect();
    let length = chars.len();
    let mut path = vec![];
    dfs_bcktrace(&chars, 0, length, &mut path, &mut res);
    res
}

fn dfs_bcktrace(chars: &Vec<u8>, index: usize, length: usize, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
    if index == length {
        res.push(path.clone());
        return;
    }
    for i in index..length {
        if !is_palindrome(chars, index, i) {
            continue;
        }
        path.push(String::from_utf8(chars[index..=i].to_vec()).unwrap());
        dfs_bcktrace(chars, i + 1, length, path, res);
        path.pop();
    }
}

fn is_palindrome(s: &Vec<u8>, mut left: usize, mut right: usize) -> bool {
    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}


pub fn partition_dp(s: String) -> Vec<Vec<String>> {
    let mut res = vec![];
    let chars: Vec<u8> = s.bytes().collect();
    let length = chars.len();
    let mut path = vec![];
    let mut dp: Vec<Vec<bool>> = vec![vec![false; length]; length];

    for right in 0..length {
        for left in 0..=right {
            if chars[left] == chars[right] && (right <= 2 + left || dp[left+1][right-1]) {
                dp[left][right] = true;
            }
        }
    }

    dfs_dp(&chars, 0, length, &dp, &mut path, &mut res);
    res
}
fn dfs_dp(chars: &Vec<u8>, index: usize, length: usize, dp: &Vec<Vec<bool>>, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
    if index == length {
        res.push(path.clone());
        return;
    }

    for i in index..length {
        if dp[index][i] {
            path.push(String::from_utf8(chars[index..=i].to_vec()).unwrap());
            dfs_dp(chars, i + 1, length, dp, path, res);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::partition_backtrace;
    use crate::partition_dp;

    #[test]
    fn it_works() {
        // assert_eq!(partition_backtrace("aab".to_string()), vec![
        //     vec!["aa".to_string(),"b".to_string()],
        //     vec!["a".to_string(),"a".to_string(),"b".to_string()]
        // ]);
        assert_eq!(partition_dp("aab".to_string()), vec![
            vec!["aa".to_string(),"b".to_string()],
            vec!["a".to_string(),"a".to_string(),"b".to_string()]
        ]);
    }
}
