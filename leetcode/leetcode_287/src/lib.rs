/**
给定一个包含 n + 1 个整数的数组 nums ，其数字都在 1 到 n 之间（包括 1 和 n），可知至少存在一个重复的整数。

假设 nums 只有 一个重复的整数 ，找出 这个重复的数 。

示例 1：
输入：nums = [1,3,4,2,2]
输出：2

示例 2：
输入：nums = [3,1,3,4,2]
输出：3

示例 3：
输入：nums = [1,1]
输出：1

示例 4：
输入：nums = [1,1,2]
输出：1

提示：
2 <= n <= 3 * 104
nums.length == n + 1
1 <= nums[i] <= n
nums 中 只有一个整数 出现 两次或多次 ，其余整数均只出现 一次
 
进阶：
如何证明 nums 中至少存在一个重复的数字?
你可以在不修改数组 nums 的情况下解决这个问题吗？
你可以只用常量级 O(1) 的额外空间解决这个问题吗？
你可以设计一个时间复杂度小于 O(n2) 的解决方案吗？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-the-duplicate-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashSet;

pub fn find_duplicate_hash(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    for i in nums {
        if set.contains(&i) {
            return i;
        }
        set.insert(i);
    }
    0
}

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = 0;
    let mut fast = 0;
    while (slow == 0 && fast == 0) || slow != fast {
        slow = nums[slow] as usize;
        fast = nums[nums[fast as usize] as usize] as usize;
    }
    slow = 0;
    while slow != fast {
        println!("s: {}, {}", slow, fast);
        slow = nums[slow] as usize;
        fast = nums[fast] as usize;
        println!("s: {}, {}", slow, fast);
    }
    slow as i32
}

#[cfg(test)]
mod tests {
    use crate::find_duplicate;

    #[test]
    fn it_works() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }
}
