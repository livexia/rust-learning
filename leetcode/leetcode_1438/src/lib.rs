/**
给你一个整数数组 nums ，和一个表示限制的整数 limit，请你返回最长连续子数组的长度，该子数组中的任意两个元素之间的绝对差必须小于或者等于 limit 。

如果不存在满足条件的子数组，则返回 0 。

示例 1：
输入：nums = [8,2,4,7], limit = 4
输出：2 
解释：所有子数组如下：
[8] 最大绝对差 |8-8| = 0 <= 4.
[8,2] 最大绝对差 |8-2| = 6 > 4. 
[8,2,4] 最大绝对差 |8-2| = 6 > 4.
[8,2,4,7] 最大绝对差 |8-2| = 6 > 4.
[2] 最大绝对差 |2-2| = 0 <= 4.
[2,4] 最大绝对差 |2-4| = 2 <= 4.
[2,4,7] 最大绝对差 |2-7| = 5 > 4.
[4] 最大绝对差 |4-4| = 0 <= 4.
[4,7] 最大绝对差 |4-7| = 3 <= 4.
[7] 最大绝对差 |7-7| = 0 <= 4. 
因此，满足题意的最长子数组的长度为 2 。

示例 2：
输入：nums = [10,1,2,4,7,2], limit = 5
输出：4 
解释：满足题意的最长子数组是 [2,4,7,2]，其最大绝对差 |2-7| = 5 <= 5 。

示例 3：
输入：nums = [4,2,2,2,4,4,2,2], limit = 0
输出：3

提示：

1 <= nums.length <= 10^5
1 <= nums[i] <= 10^9
0 <= limit <= 10^9

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::VecDeque;

pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut max_deque: VecDeque<i32> = VecDeque::new();
    let mut min_deque: VecDeque<i32> = VecDeque::new();
    let len = nums.len();
    let mut res  = 0;

    let mut left = 0;
    let mut right = 0;

    while right < len {
        while !max_deque.is_empty() && max_deque.back().unwrap() < &nums[right] {
            max_deque.pop_back();
        }
        while !min_deque.is_empty() && min_deque.back().unwrap() > &nums[right] {
            min_deque.pop_back();
        }
        max_deque.push_back(nums[right]);
        min_deque.push_back(nums[right]);
        while !max_deque.is_empty() && !min_deque.is_empty() && max_deque.front().unwrap() - min_deque.front().unwrap() > limit {
            if &nums[left] == max_deque.front().unwrap() {
                max_deque.pop_front();
            }
            if &nums[left] == min_deque.front().unwrap() {
                min_deque.pop_front();
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
    use crate::longest_subarray;

    #[test]
    fn it_works() {
        assert_eq!(longest_subarray(vec![4,2,2,2,4,4,2,2], 0), 3);
    }
}
