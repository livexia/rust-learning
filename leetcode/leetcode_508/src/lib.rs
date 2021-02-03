/*
给你一个二叉树的根结点，请你找出出现次数最多的子树元素和。一个结点的「子树元素和」定义为以该结点为根的二叉树上所有结点的元素之和（包括结点本身）。

你需要返回出现次数最多的子树元素和。如果有多个元素出现的次数相同，返回所有出现次数最多的子树元素和（不限顺序）。

示例 1：
输入:

  5
 /  \
2   -3
返回 [2, -3, 4]，所有的值均只出现一次，以任意顺序返回所有值。

示例 2：
输入：

  5
 /  \
2   -5
返回 [2]，只有 2 出现两次，-5 只出现 1 次。

提示： 假设任意子树元素和均可以用 32 位有符号整数表示。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/most-frequent-subtree-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashMap;

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

pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map = HashMap::new();

        dfs(root, &mut map);
        let mut raw: Vec<_> = map.iter().collect();
        raw.sort_by(|a,b| (b.1).cmp(a.1));
        if raw.len() < 1 {
            return vec![]
        }
        let max = raw[0].1;
        raw.iter().filter(|i| i.1 == max).map(|i| *i.0).collect::<Vec<i32>>()
}

pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) -> i32 {
    match node {
        Some(n) => {
            let mut sum = n.borrow().val;
            sum += dfs(n.borrow().left.clone(), map);
            sum += dfs(n.borrow().right.clone(), map);
            let count = map.entry(sum).or_insert(0);
            *count += 1;
            sum
        },
        None => 0
    }
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
