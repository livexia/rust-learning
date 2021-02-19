/**
给定一个由若干 0 和 1 组成的数组 A，我们最多可以将 K 个值从 0 变成 1 。

返回仅包含 1 的最长（连续）子数组的长度。

示例 1：
输入：A = [1,1,1,0,0,0,1,1,1,1,0], K = 2
输出：6
解释： 
[1,1,1,0,0,1,1,1,1,1,1]
粗体数字从 0 翻转到 1，最长的子数组长度为 6。

示例 2：
输入：A = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], K = 3
输出：10
解释：
[0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
粗体数字从 0 翻转到 1，最长的子数组长度为 10。

提示：
1 <= A.length <= 20000
0 <= K <= A.length
A[i] 为 0 或 1 

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/max-consecutive-ones-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;

    let len = a.len();
    let mut left = 0;
    let mut right = 0;
    let mut zero_count = 0;

    while right < len {
        if a[right] == 0 {
            zero_count += 1;
        }
        while zero_count > k {
            if a[left] == 0 {
                zero_count -= 1;
            }
            left += 1;
        }
        res = res.max(right - left + 1);
        right += 1;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use crate::longest_ones;

    #[test]
    fn t1() {
        assert_eq!(longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2), 6);
    }

    #[test]
    fn t2() {
        assert_eq!(longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3), 10);
    }
}
