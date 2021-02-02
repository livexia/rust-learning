use core::num;

/*
设计一个算法，找出数组中两数之和为指定值的所有整数对。一个数只能属于一个数对。

示例 1:
输入: nums = [5,6,5], target = 11
输出: [[5,6]]

示例 2:
输入: nums = [5,6,5,6], target = 11
输出: [[5,6],[5,6]]

提示：
nums.length <= 100000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/pairs-with-sum-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn pair_sums(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
        return vec![]
    }
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    
    let mut left = 0;
    let mut right = nums.len() - 1;

    let mut res= Vec::new();

    while left < right {
        let sum = sorted_nums[left] + sorted_nums[right];
        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            res.push(vec![sorted_nums[left], sorted_nums[right]]);
            left += 1;
            right -= 1;
        }
    }
    res
}

#[test]
fn it_works() {
    assert_eq!(pair_sums(vec![5, 6, 5], 11), vec![vec![5, 6]]);
}
