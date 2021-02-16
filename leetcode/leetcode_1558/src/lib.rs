/**
[func](https://assets.leetcode.com/uploads/2020/07/10/sample_2_1887.png)
给你一个与 nums 大小相同且初始值全为 0 的数组 arr ，请你调用以上函数得到整数数组 nums 。

请你返回将 arr 变成 nums 的最少函数调用次数。

答案保证在 32 位有符号整数以内。

 

示例 1：

输入：nums = [1,5]
输出：5
解释：给第二个数加 1 ：[0, 0] 变成 [0, 1] （1 次操作）。
将所有数字乘以 2 ：[0, 1] -> [0, 2] -> [0, 4] （2 次操作）。
给两个数字都加 1 ：[0, 4] -> [1, 4] -> [1, 5] （2 次操作）。
总操作次数为：1 + 2 + 2 = 5 。
示例 2：

输入：nums = [2,2]
输出：3
解释：给两个数字都加 1 ：[0, 0] -> [0, 1] -> [1, 1] （2 次操作）。
将所有数字乘以 2 ： [1, 1] -> [2, 2] （1 次操作）。
总操作次数为： 2 + 1 = 3 。
示例 3：

输入：nums = [4,2,5]
输出：6
解释：（初始）[0,0,0] -> [1,0,0] -> [1,0,1] -> [2,0,2] -> [2,1,2] -> [4,2,4] -> [4,2,5] （nums 数组）。
示例 4：

输入：nums = [3,2,2,4]
输出：7
示例 5：

输入：nums = [2,4,8,16]
输出：8
 

提示：

1 <= nums.length <= 10^5
0 <= nums[i] <= 10^9

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-numbers-of-function-calls-to-make-target-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    let target = vec![0; len];
    let mut res = 0;

    while nums != target {
        nums = nums.iter().map(|x| if x % 2 == 1 { res+=1; (x - 1) / 2 } else { x / 2 } ).collect();
        if nums == target {
            break;
        }
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::min_operations;

    #[test]
    fn it_works() {
        assert_eq!(min_operations(vec![3, 2, 2, 4]), 7);
    }
}
