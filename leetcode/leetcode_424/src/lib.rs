/*
给你一个仅由大写英文字母组成的字符串，你可以将任意位置上的字符替换成另外的字符，总共可最多替换 k 次。在执行上述操作后，找到包含重复字母的最长子串的长度。

注意：字符串长度 和 k 不会超过 104。

示例 1：
输入：s = "ABAB", k = 2
输出：4
解释：用两个'A'替换为两个'B',反之亦然。

示例 2：
输入：s = "AABABBA", k = 1
输出：4
解释：
将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
子串 "BBBB" 有最长重复字母, 答案为 4。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-repeating-character-replacement
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn character_replacement(s: String, k: i32) -> i32 {
    let l = s.len();
    if l < 2 {
        return l as i32;
    }
    let mut max = 0;
    let mut freq = vec![0; 26];

    let mut left: usize = 0;
    let mut right: usize = 0;

    while right < l {
        freq[s.chars().nth(right).unwrap() as usize - 'A' as usize] += 1;
        max = max.max(freq[s.chars().nth(right).unwrap() as usize - 'A' as usize]);
        if right - left + 1 > max + k as usize {
            freq[s.chars().nth(left).unwrap() as usize - 'A' as usize] -= 1;
            left += 1;
        }
        right += 1;
    }
    (right - left) as i32
}

#[test]
fn it_works() {
    assert_eq!(4, character_replacement("AABABBA".to_string(), 1));
}