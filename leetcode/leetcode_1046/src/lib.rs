/**
有一堆石头，每块石头的重量都是正整数。

每一回合，从中选出两块 最重的 石头，然后将它们一起粉碎。假设石头的重量分别为 x 和 y，且 x <= y。那么粉碎的可能结果如下：

如果 x == y，那么两块石头都会被完全粉碎；
如果 x != y，那么重量为 x 的石头将会完全粉碎，而重量为 y 的石头新重量为 y-x。
最后，最多只会剩下一块石头。返回此石头的重量。如果没有石头剩下，就返回 0。

示例：
输入：[2,7,4,1,8,1]
输出：1
解释：
先选出 7 和 8，得到 1，所以数组转换为 [2,4,1,1,1]，
再选出 2 和 4，得到 2，所以数组转换为 [2,1,1,1]，
接着是 2 和 1，得到 1，所以数组转换为 [1,1,1]，
最后选出 1 和 1，得到 0，最终数组转换为 [1]，这就是最后剩下那块石头的重量。
 
提示：

1 <= stones.length <= 30
1 <= stones[i] <= 1000


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/last-stone-weight
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::fmt::Debug;

#[derive(Clone)]
pub struct MaxHeap<T> {
    data: Vec<T>,
}

impl<T> MaxHeap<T>
where
    T: Clone + Copy + PartialEq + Eq + Ord + Debug,
{
    fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    fn from_vec(data: &Vec<T>) -> Self {
        let mut heap = MaxHeap { data: data.clone() };
        heap.build_max_heap();
        heap
    }

    fn build_max_heap(&mut self) {
        let n = self.len();
        for i in (0..n / 2).rev() {
            self.max_heapify(i)
        }
    }

    fn max_heapify(&mut self, i: usize) {
        let n = self.len();
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let mut largest = i;
        if l < n && self.data[l] > self.data[i] {
            largest = l;
        }
        if r < n && self.data[r] > self.data[largest] {
            largest = r;
        }
        if largest != i {
            self.swap(i, largest);
            self.max_heapify(largest);
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b)
    }

    fn pop_max(&mut self) -> Option<T>{
        let n = self.len();
        self.swap(0, n - 1);
        let result = self.data.pop();
        self.max_heapify(0);
        result
    }

    fn sort(&self) -> Vec<T> {
        let mut heap = self.clone();
        let mut result = vec![];
        while !heap.is_empty() {
            result.push(heap.pop_max().unwrap());
        }
        result
    }

    fn push(&mut self, a: T) {
        self.data.push(a);
        let mut cur = self.len() - 1;
        while cur != 0 {
            let parent = (cur - 1) / 2;
            if self.data[cur] > self.data[parent] {
                self.swap(cur, parent);
                cur = parent;
            } else {
                break;
            }
        }
    }
}

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = MaxHeap::from_vec(&stones);
    while heap.len() > 1 {
        let a = heap.pop_max().unwrap();
        let b = heap.pop_max().unwrap();
        if a == b {
            continue;
        } else {
            heap.push((a-b).abs());
        }
    }
    if heap.is_empty() {
        0
    } else {
        heap.pop_max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::last_stone_weight;

    #[test]
    fn it_works() {
        assert_eq!(last_stone_weight(vec![2,7,4,1,8,1]), 1);
    }
}
