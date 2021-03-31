/**
返回与给定的前序和后序遍历匹配的任何二叉树。

 pre 和 post 遍历中的值是不同的正整数。

示例：
输入：pre = [1,2,4,5,3,6,7], post = [4,5,2,6,7,3,1]
输出：[1,2,3,4,5,6,7]

提示：
1 <= pre.length == post.length <= 30
pre[] 和 post[] 都是 1, 2, ..., pre.length 的排列
每个输入保证至少有一个答案。如果有多个答案，可以返回其中一个。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal
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

pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    dfs(&pre[..], &post[..])
}

fn dfs(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if pre.is_empty() {
        return None;
    }
    let mut root = TreeNode::new(pre[0]);
    if pre.len() == 1 {
        return Some(Rc::new(RefCell::new(root)))
    }
    let left_count = post.iter().position(|&n| n == pre[1]).unwrap() + 1;
    let last = post.len() - 1;
    root.left = dfs(&pre[1..left_count+1], &post[..left_count]);
    root.right = dfs(&pre[left_count+1..], &post[left_count..last]);
    Some(Rc::new(RefCell::new(root)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
