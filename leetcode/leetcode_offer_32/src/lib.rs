/*
请实现一个函数按照之字形顺序打印二叉树，即第一行按照从左到右的顺序打印，第二层按照从右到左的顺序打印，第三行再按照从左到右的顺序打印，其他行以此类推。

例如:
给定二叉树: [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7
返回其层次遍历结果：

[
  [3],
  [20,9],
  [15,7]
]
 

提示：
节点总数 <= 1000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::{collections::VecDeque, rc::Rc};
use std::cell::RefCell;
use std::collections::VecDeque;

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



pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];

    if root.is_none() {
        return res;
    }

    let mut d = VecDeque::new();

    let root = root.unwrap();
    let mut rev = true;
    d.push_back(root);

    while !d.is_empty() {
        let mut temp: Vec<i32> = vec![];
        let size = d.len();
        for _ in 0..size {
            let node = d.pop_front().unwrap();
            let val = node.borrow().val;
            if rev {
                temp.push(val)
            } else {
                temp.insert(0, val)
            }
            if let Some(left) = node.borrow_mut().left.take() {
                d.push_back(left)
            }
            if let Some(right) = node.borrow_mut().right.take() {
                d.push_back(right)
            }
        }
        if temp.len() > 0 {
            res.push(temp)
        }
        rev = !rev;
    }
    res
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
