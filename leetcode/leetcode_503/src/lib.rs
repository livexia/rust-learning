use core::num;

/**
给定一个循环数组（最后一个元素的下一个元素是数组的第一个元素），输出每个元素的下一个更大元素。数字 x 的下一个更大的元素是按数组遍历顺序，这个数字之后的第一个比它更大的数，这意味着你应该循环地搜索它的下一个更大的数。如果不存在，则输出 -1。

示例 1:
输入: [1,2,1]
输出: [2,-1,2]
解释: 第一个 1 的下一个更大的数是 2；
数字 2 找不到下一个更大的数； 
第二个 1 的下一个最大的数需要循环搜索，结果也是 2。
注意: 输入数组的长度不会超过 10000。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/next-greater-element-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
pub fn next_greater_elements_brute_force(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![];
    'outer: for i in 0..n {
        let temp = nums[i];
        for j in i..n {
            if temp < nums[j] {
                ans.push(nums[j]);
                continue 'outer;
            }
        }
        for j in 0..i {
            if temp < nums[j] {
                ans.push(nums[j]);
                continue 'outer;
            }
        }
        ans.push(-1)
    }
    ans
}

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() { return vec![]; }
    let n = nums.len();
    let mut ans = vec![-1; n];
    let mut stack = vec![];
    for i in 0..2*n-1 {
        while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i % n] {
            ans[stack.pop().unwrap()] = nums[i % n];
        }
        stack.push(i % n);
    }
    ans

}

#[cfg(test)]
mod tests {
    use crate::next_greater_elements;
    use crate::next_greater_elements_brute_force;

    #[test]
    fn it_works() {
        assert_eq!(next_greater_elements_brute_force(vec![1,2,1]), next_greater_elements(vec![1,2,1]));
    }
}
