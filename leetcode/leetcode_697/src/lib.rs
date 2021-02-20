/**
给定一个非空且只包含非负数的整数数组 nums，数组的度的定义是指数组里任一元素出现频数的最大值。

你的任务是在 nums 中找到与 nums 拥有相同大小的度的最短连续子数组，返回其长度。

示例 1：
输入：[1, 2, 2, 3, 1]
输出：2
解释：
输入数组的度是2，因为元素1和2的出现频数最大，均为2.
连续子数组里面拥有相同度的有如下所示:
[1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
最短连续子数组[2, 2]的长度为2，所以返回2.

示例 2：
输入：[1,2,2,3,1,4,2]
输出：6
 
提示：
nums.length 在1到 50,000 区间范围内。
nums[i] 是一个在 0 到 49,999 范围内的整数。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/degree-of-an-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut freq = std::collections::HashMap::new();
    nums.iter().for_each(|&x| *freq.entry(x).or_insert(0) += 1);
    let degree = freq.into_iter().map(|(k, v)| v).max().unwrap();
    let n = nums.len();

    let mut answer = n + 1;
    let mut window_freq = std::collections::HashMap::new();
    let mut left = 0;
    for right in 0..n {
        let count = window_freq.entry(nums[right]).or_insert(0);
        *count += 1;
        if *count < degree {
            continue;
        }
        while nums[right] != nums[left] {
            *window_freq.get_mut(&nums[left]).unwrap() -= 1;
            left += 1;
        }
        answer = answer.min(right - left + 1);
    }
    answer as i32
}

#[cfg(test)]
mod tests {
    use crate::find_shortest_sub_array;

    #[test]
    fn it_works() {
        assert_eq!(find_shortest_sub_array(vec![1,2,2,3,1,4,2]), 6);
    }
}
