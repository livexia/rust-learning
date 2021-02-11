/**
设计一个找到数据流中第 k 大元素的类（class）。注意是排序后的第 k 大元素，不是第 k 个不同的元素。

请实现 KthLargest 类：

KthLargest(int k, int[] nums) 使用整数 k 和整数流 nums 初始化对象。
int add(int val) 将 val 插入数据流 nums 后，返回当前数据流中第 k 大的元素。

示例：
输入：
["KthLargest", "add", "add", "add", "add", "add"]
[[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
输出：
[null, 4, 5, 5, 8, 8]

解释：
KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
kthLargest.add(3);   // return 4
kthLargest.add(5);   // return 5
kthLargest.add(10);  // return 5
kthLargest.add(9);   // return 8
kthLargest.add(4);   // return 8
 
提示：
1 <= k <= 104
0 <= nums.length <= 104
-104 <= nums[i] <= 104
-104 <= val <= 104
最多调用 add 方法 104 次
题目数据保证，在查找第 k 大元素时，数组中至少有 k 个元素

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/kth-largest-element-in-a-stream
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    min_heap_len: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a muta ble reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut r = KthLargest {
            min_heap: BinaryHeap::new(),
            min_heap_len: k as usize,
        };
        nums.into_iter().for_each(|x| { KthLargest::add(&mut r, x); });
        r
    }
    
    fn add(&mut self, val: i32) -> i32 {
        println!("{:?}", self);
        if self.min_heap.len() < self.min_heap_len {
            self.min_heap.push(Reverse(val));
        } else if self.min_heap.peek().unwrap().0 < val {
            self.min_heap.pop();
            self.min_heap.push(Reverse(val));
        }
        self.min_heap.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use crate::KthLargest;

    #[test]
    fn it_works() {
        let mut r = KthLargest::new(3, vec![4, 5, 8, 2]);

        assert_eq!(4, r.add(3));
        assert_eq!(5, r.add(5));
        assert_eq!(5, r.add(10));
        assert_eq!(8, r.add(9));
        assert_eq!(8, r.add(4));
    }
}
