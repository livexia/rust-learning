/**
给你一个由非负整数组成的数组 nums 。另有一个查询数组 queries ，其中 queries[i] = [xi, mi] 。

第 i 个查询的答案是 xi 和任何 nums 数组中不超过 mi 的元素按位异或（XOR）得到的最大值。换句话说，答案是 max(nums[j] XOR xi) ，其中所有 j 均满足 nums[j] <= mi 。如果 nums 中的所有元素都大于 mi，最终答案就是 -1 。

返回一个整数数组 answer 作为查询的答案，其中 answer.length == queries.length 且 answer[i] 是第 i 个查询的答案。

 

示例 1：

输入：nums = [0,1,2,3,4], queries = [[3,1],[1,3],[5,6]]
输出：[3,3,7]
解释：
1) 0 和 1 是仅有的两个不超过 1 的整数。0 XOR 3 = 3 而 1 XOR 3 = 2 。二者中的更大值是 3 。
2) 1 XOR 2 = 3.
3) 5 XOR 2 = 7.
示例 2：

输入：nums = [5,2,4,6,6,3], queries = [[12,4],[8,1],[6,3]]
输出：[15,-1,5]
 

提示：

1 <= nums.length, queries.length <= 105
queries[i].length == 2
0 <= nums[j], xi, mi <= 109


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-xor-with-an-element-from-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

#[derive(Default)]
pub struct Trie {
    root: Node
}

#[derive(Default)]
pub struct Node {
    children: [Option<Box<Node>>; 2]
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, num: i32) {
        let mut node = &mut self.root;
        for i in 0..32 {
            let idx = ((num >> (31 - i)) & 1) as usize;
            let next = &mut node.children[idx];
            node = next.get_or_insert_with(Box::<Node>::default)
        }
    }

    pub fn get_max_xor_sum(&self, num: i32) -> i32 {
        let mut node = &self.root;
        let mut res = 0;
        for i in 0..32 {
            let mut idx = (((num >> (31 - i)) & 1) ^ 1) as usize;
            if node.children[idx].is_none() {
                idx ^= 1;
            }
            if let Some(next) = &node.children[idx] {
                node = next.as_ref();
                res = res * 2 + idx as i32;
            }
        }
        res
    }
}

pub fn maximize_xor(mut nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    nums.sort_by_key(|x| -x);
    for (index, query) in queries.iter_mut().enumerate() {
        query.push(index as i32);
    }
    queries.sort_by_key(|x| x[1]);
    let mut res = vec![0; queries.len()];
    let mut trie = Trie::new();
    for query in queries {
        let (x, m, pos) = (query[0], query[1], query[2] as usize);
        while nums.len() > 0 && nums.last().unwrap() <= &m {
            trie.insert(nums.pop().unwrap());
        }
        if nums.len() < n {
            res[pos] = trie.get_max_xor_sum(x) ^ x;
        } else {
            res[pos] = -1
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
