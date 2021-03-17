/**
给定二叉搜索树的根结点 root，返回值位于范围 [low, high] 之间的所有结点的值的和。

示例 1：
输入：root = [10,5,15,3,7,null,18], low = 7, high = 15
输出：30

示例 2：
输入：root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
输出：23

提示：

树中节点数目在范围 [1, 2 * 104] 内
1 <= Node.val <= 105
1 <= low <= high <= 105
所有 Node.val 互不相同

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/range-sum-of-bst
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

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut answer = 0;
    in_order(&root, &mut answer, low, high);
    answer
}

fn in_order(root: &Option<Rc<RefCell<TreeNode>>>, answer: &mut i32, low: i32, high: i32) {
    match root {
        Some(node) => {
            if low <= node.borrow().val && node.borrow().val <= high {
                *answer += node.borrow().val;
            }
            if low < node.borrow().val {
                in_order(&node.borrow().left, answer, low, high);
            }
            if node.borrow().val < high {
                in_order(&node.borrow().right, answer, low, high);
            }
        },
        None => return,
    }
}
    

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
