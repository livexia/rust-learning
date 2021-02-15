/**
给定一个二进制数组， 计算其中最大连续1的个数。

示例 1:
输入: [1,1,0,1,1,1]
输出: 3
解释: 开头的两位和最后的三位都是连续1，所以最大连续1的个数是 3.
注意：
输入的数组只包含 0 和1。
输入数组的长度是正整数，且不超过 10,000。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/max-consecutive-ones
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut res = 0;

    let mut left = 0;
    let mut right = 0;

    while right < nums.len() {
        if nums[right] == 1 {
            right += 1;
        } else {
            right += 1;
            left = right;
            continue;
        }
        res = res.max(right - left)
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use crate::find_max_consecutive_ones;

    #[test]
    fn t1() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1, 1]), 4);
    }

    #[test]
    fn t2() {
        assert_eq!(find_max_consecutive_ones(vec![0]), 0);
    }
}
