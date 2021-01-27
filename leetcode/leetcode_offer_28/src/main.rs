/*
请实现一个函数，用来判断一棵二叉树是不是对称的。如果一棵二叉树和它的镜像一样，那么它是对称的。

例如，二叉树 [1,2,2,3,4,4,3] 是对称的。

    1
   / \
  2   2
 / \ / \
3  4 4  3
但是下面这个 [1,2,2,null,3,null,3] 则不是镜像对称的:

    1
   / \
  2   2
   \   \
   3    3

示例 1：
输入：root = [1,2,2,3,4,4,3]
输出：true

示例 2：
输入：root = [1,2,2,null,3,null,3]
输出：false

限制：

0 <= 节点个数 <= 1000

注意：本题与主站 101 题相同：https://leetcode-cn.com/problems/symmetric-tree/

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/dui-cheng-de-er-cha-shu-lcof
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::rc::Rc;
use std::cell::RefCell;

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

fn main() {
    println!("Hello, world!");
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(root) = root {
        let r = root.borrow();
        let a = r.left.as_ref();
        let b = r.right.as_ref();
        match (a, b) {
            (Some(a), Some(b)) => {
                check_symmetric(Some(a), Some(b))
            },
            (None, None) => { true },
            _ => { false },
        }
    } else {
        true
    }
}
fn check_symmetric(a: Option<&Rc<RefCell<TreeNode>>>, b: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => {
            if a.borrow().val != b.borrow().val {
                return false;
            }
            return check_symmetric(a.borrow().left.as_ref(), b.borrow().right.as_ref()) && 
                   check_symmetric(a.borrow().right.as_ref(), b.borrow().left.as_ref());
        },
        (None, None) => { true },
        _ => { false },
    }
}
