/**
给定一个字符串数组 arr，字符串 s 是将 arr 某一子序列字符串连接所得的字符串，如果 s 中的每一个字符都只出现过一次，那么它就是一个可行解。

请返回所有可行解 s 中最长长度。

示例 1：
输入：arr = ["un","iq","ue"]
输出：4
解释：所有可能的串联组合是 "","un","iq","ue","uniq" 和 "ique"，最大长度为 4。

示例 2：
输入：arr = ["cha","r","act","ers"]
输出：6
解释：可能的解答有 "chaers" 和 "acters"。

示例 3：
输入：arr = ["abcdefghijklmnopqrstuvwxyz"]
输出：26

提示：

1 <= arr.length <= 16
1 <= arr[i].length <= 26
arr[i] 中只含有小写英文字母

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn max_length(arr: Vec<String>) -> i32 {
    let mut masks = vec![];
    for s in arr {
        let mut mask = 0;
        for &byte in s.as_bytes() {
            if mask >> byte & 1 != 0 {
                mask = 0;
                break;
            }
            mask |= 1 << byte;
        }
        if mask > 0 {
            masks.push(mask);
        }
    }
    backtrace(&masks, 0, 0, 0)
}

pub fn backtrace(masks: &Vec<i32>, pos: usize, mask: i32, mut ans: i32) -> i32 {
    if pos == masks.len() {
        ans = ans.max(mask.count_ones() as i32);
        return ans;
    }
    if mask & masks[pos] == 0 {
        ans = backtrace(masks, pos + 1, mask | masks[pos], ans);
    }
    backtrace(masks, pos + 1, mask, ans)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
