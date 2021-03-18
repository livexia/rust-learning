/**
给你一棵二叉树，每个节点的值为 1 到 9 。我们称二叉树中的一条路径是 「伪回文」的，当它满足：路径经过的所有节点值的排列中，存在一个回文序列。

请你返回从根到叶子节点的所有路径中 伪回文 路径的数目。

示例 1：
输入：root = [2,3,1,3,1,null,1]
输出：2 
解释：上图为给定的二叉树。总共有 3 条从根到叶子的路径：红色路径 [2,3,3] ，绿色路径 [2,1,1] 和路径 [2,3,1] 。
     在这些路径中，只有红色和绿色的路径是伪回文路径，因为红色路径 [2,3,3] 存在回文排列 [3,2,3] ，绿色路径 [2,1,1] 存在回文排列 [1,2,1] 。

示例 2：
输入：root = [2,1,1,1,3,null,null,null,null,null,1]
输出：1 
解释：上图为给定二叉树。总共有 3 条从根到叶子的路径：绿色路径 [2,1,1] ，路径 [2,1,3,1] 和路径 [2,1] 。
     这些路径中只有绿色路径是伪回文路径，因为 [2,1,1] 存在回文排列 [1,2,1] 。

示例 3：
输入：root = [9]
输出：1

提示：
给定二叉树的节点数目在 1 到 10^5 之间。
节点值在 1 到 9 之间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/pseudo-palindromic-paths-in-a-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut freq = vec![0; 10];
    dfs(&root, &mut freq)
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, freq: &mut Vec<i32>) -> i32 {
    match node {
        Some(node) => {
            let mut ans = 0;
            let val = node.borrow().val as usize;
            freq[val] += 1;
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                if is_palindromic(freq) {
                    ans += 1;
                }
                freq[val] -= 1;
                return ans;
            }
            ans += dfs(&node.borrow().left, freq);
            ans += dfs(&node.borrow().right, freq);
            freq[val] -= 1;
            return ans;
        },
        None => {
            return 0;
        },
    }
}

fn is_palindromic(freq: &Vec<i32>) -> bool {
    freq.iter().skip(1).fold(0, |count, &v| {
        if v % 2 == 1 { count + 1 } else { count }
    }) < 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
