use std::collections::VecDeque;

/*
中位数是有序序列最中间的那个数。如果序列的大小是偶数，则没有最中间的数；此时中位数是最中间的两个数的平均数。

例如：
[2,3,4]，中位数是 3
[2,3]，中位数是 (2 + 3) / 2 = 2.5
给你一个数组 nums，有一个大小为 k 的窗口从最左端滑动到最右端。窗口中有 k 个数，每次窗口向右移动 1 位。你的任务是找出每次窗口移动后得到的新窗口中元素的中位数，并输出由它们组成的数组。

示例：

给出 nums = [1,3,-1,-3,5,3,6,7]，以及 k = 3。

窗口位置                      中位数
---------------               -----
[1  3  -1] -3  5  3  6  7       1
 1 [3  -1  -3] 5  3  6  7      -1
 1  3 [-1  -3  5] 3  6  7      -1
 1  3  -1 [-3  5  3] 6  7       3
 1  3  -1  -3 [5  3  6] 7       5
 1  3  -1  -3  5 [3  6  7]      6
 因此，返回该滑动窗口的中位数数组 [1,-1,-1,3,5,6]。

提示：
你可以假设 k 始终有效，即：k 始终小于输入的非空数组的元素个数。
与真实值误差在 10 ^ -5 以内的答案将被视作正确答案。


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sliding-window-median
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

// pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
//     let mut res: Vec<f64> = Vec::new();
//     let offset = k as usize;
//     for i in 0..nums.len() - offset + 1 {
//         let mut n = nums[i..i+offset].to_vec();
//         n.sort();
//         res.push(get_mid(&n));
//     }
//     res
// }

pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let k = k as usize;
    let (mut windows, mut sorted_windows) = (VecDeque::with_capacity(k), Vec::with_capacity(k));
    let mut medians = Vec::with_capacity(nums.len().max(k) - k + 1);
    for n in nums {
        windows.push_back(n);
        sorted_windows.insert(sorted_windows.binary_search(&n).unwrap_or_else(|i| i), n);
        if sorted_windows.len() < k {
            continue;
        }
        if sorted_windows.len() > k {
            sorted_windows.remove(sorted_windows.binary_search(&windows.pop_front().unwrap()).unwrap());
        }
        medians.push(get_mid(&sorted_windows));
    }
    medians
}

pub fn get_mid(nums: &[i32]) -> f64 {
    (nums[(nums.len()-1) / 2] as f64 + nums[nums.len() / 2] as f64) / 2.0
}

#[test]
fn it_works() {
    assert_eq!(vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0], median_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3));
}
