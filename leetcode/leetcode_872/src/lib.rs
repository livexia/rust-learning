/**
请考虑一棵二叉树上所有的叶子，这些叶子的值按从左到右的顺序排列形成一个 叶值序列 。

举个例子，如上图所示，给定一棵叶值序列为 (6, 7, 4, 9, 8) 的树。

如果有两棵二叉树的叶值序列是相同，那么我们就认为它们是 叶相似 的。

如果给定的两个根结点分别为 root1 和 root2 的树是叶相似的，则返回 true；否则返回 false 。

示例 1：
输入：root1 = [3,5,1,6,2,9,8,null,null,7,4], root2 = [3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]
输出：true

示例 2：
输入：root1 = [1], root2 = [1]
输出：true
示例 3：
输入：root1 = [1], root2 = [2]
输出：false

示例 4：
输入：root1 = [1,2], root2 = [2,2]
输出：true
示例 5：
输入：root1 = [1,2,3], root2 = [1,3,2]
输出：false
 
提示：
给定的两棵树可能会有 1 到 200 个结点。
给定的两棵树上的值介于 0 到 200 之间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/leaf-similar-trees
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

use std::{borrow::Borrow, rc::Rc};
use std::cell::RefCell;

pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn leaf(node: &Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
        if let Some(node) = node {
            let left = &node.as_ref().borrow().left;
            let right = &node.as_ref().borrow().right;
            leaf(&left, leaves);
            leaf(&right, leaves);
            if left.is_none() && right.is_none() {
                leaves.push(node.as_ref().borrow().val)
            }
        }
    }
    let mut leaves1 = vec![];
    let mut leaves2 = vec![];
    leaf(&root1, &mut leaves1);
    leaf(&root2, &mut leaves2);
    leaves1 == leaves2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
