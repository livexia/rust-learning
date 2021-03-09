/**
给定一个非空的整数数组，返回其中出现频率前 k 高的元素。

示例 1:
输入: nums = [1,1,1,2,2,3], k = 2
输出: [1,2]

示例 2:
输入: nums = [1], k = 1
输出: [1]
 
提示：

你可以假设给定的 k 总是合理的，且 1 ≤ k ≤ 数组中不相同的元素的个数。
你的算法的时间复杂度必须优于 O(n log n) , n 是数组的大小。
题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的。
你可以按任意顺序返回答案。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/top-k-frequent-elements
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashMap;
use std::collections::BinaryHeap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut freq = HashMap::new();
    let mut heap = BinaryHeap::with_capacity(k as usize);
    
    for i in nums {
        *freq.entry(i).or_insert(0) += 1;
    }
    for (k, v) in freq {
        heap.push((v, k));
    }
    for _ in 0..k {
        res.push(heap.pop().unwrap().1);
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::top_k_frequent;

    #[test]
    fn it_works() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }
}
