/**
给你一个整数数组 nums，请你将该数组升序排列。

示例 1：
输入：nums = [5,2,3,1]
输出：[1,2,3,5]

示例 2：
输入：nums = [5,1,1,2,0,0]
输出：[0,0,1,1,2,5]

提示：

1 <= nums.length <= 50000
-50000 <= nums[i] <= 50000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sort-an-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    // let mut nums = nums;
    // nums.sort();
    // nums

    quick_sort(&nums)
}

fn quick_sort(nums: &Vec<i32>) -> Vec<i32> {
    if nums.len() < 1 {
        return nums.clone()
    }
    let mut left = vec![];
    let mut right = vec![];
    let n = nums.len();
    for i in 1..n {
        if nums[i] >= nums[0] {
            right.push(nums[i]);
        } else {
            left.push(nums[i]);
        }
    }
    let mut result = quick_sort(&left);
    result.push(nums[0]);
    result.append(&mut quick_sort(&right));
    result
}

#[cfg(test)]
mod tests {
    use crate::sort_array;

    #[test]
    fn it_works() {
        assert_eq!(sort_array(vec![5,1,1,2,0,0]), vec![0,0,1,1,2,5]);
    }
}
