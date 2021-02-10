/*
给定两个字符串 s1 和 s2，写一个函数来判断 s2 是否包含 s1 的排列。

换句话说，第一个字符串的排列之一是第二个字符串的子串。

示例1:
输入: s1 = "ab" s2 = "eidbaooo"
输出: True
解释: s2 包含 s1 的排列之一 ("ba").

示例2:
输入: s1= "ab" s2 = "eidboaoo"
输出: False
注意：
输入的字符串只包含小写字母
两个字符串的长度都在 [1, 10,000] 之间

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/permutation-in-string
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false
    }
    let size = s1.len();
    let mut target = vec![0; 26];
    let mut window = vec![0; 26];

    for i in 0..size {
        target[s1.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
        window[s2.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
    }
    let mut left = 0;
    let mut right = size;
    while right < s2.len() {
        if target == window {
            return true
        }
        window[s2.chars().nth(left).unwrap() as usize - 'a' as usize] -= 1;
        window[s2.chars().nth(right).unwrap() as usize - 'a' as usize] += 1;
        right += 1;
        left += 1;
    }
    return target == window
}

#[cfg(test)]
mod tests {
    use crate::check_inclusion;

    #[test]
    fn it_works() {
        assert_eq!(check_inclusion("ab".to_string(), "eidboaooo".to_string()), true);
    }
}
