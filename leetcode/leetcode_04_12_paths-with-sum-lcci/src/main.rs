/*
给定一棵二叉树，其中每个节点都含有一个整数数值(该值或正或负)。设计一个算法，打印节点数值总和等于某个给定值的所有路径的数量。注意，路径不一定非得从二叉树的根节点或叶节点开始或结束，但是其方向必须向下(只能从父节点指向子节点方向)。

示例:
给定如下二叉树，以及目标和 sum = 22，

              5
             / \
            4   8
           /   / \
          11  13  4
         /  \    / \
        7    2  5   1
返回:

3
解释：和为 22 的路径有：[5,4,11,2], [5,8,4,5], [4,11,7]
提示：

节点总数 <= None

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/paths-with-sum-lcci
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
    let tree = create_tree(&mut vec![Some(5), Some(4), Some(11), Some(7), None, None, Some(2), None, None, None, Some(8), Some(13), None, None, Some(4), Some(5), None, None, Some(1), None, None]);
    // let tree = create_tree(&mut vec![Some(1)]);
    // let tree = create_tree(&mut vec![Some(0), Some(1), Some(1)]);
    // let tree = create_tree(&mut vec![Some(-2), None, Some(-3)]);
    // let tree = create_tree(&mut vec![Some(1), Some(-2), None, None, Some(3)]);
    let tree = create_tree(&mut vec![Some(1), Some(-2), Some(1), Some(-1), None, None, None, Some(3), None, None, Some(-3), Some(-2)]);
    let tree = create_tree(&mut vec![Some(1), Some(0), Some(1), Some(0), None, None, Some(1), None, None, Some(2), Some(-1), None, None, Some(0), None, None, Some(1),Some(0), Some(-1), None, None, Some(0), None, None, Some(-1), Some(1), None, None, Some(0)]);
    println!("{:?}", tree);
    println!("{:?}", path_sum(tree, 2));
}

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    if root == None {
        return 0;
    }
    pre_order_travel(&root, sum, &mut Vec::new(), 0, 0)
}

pub fn pre_order_travel(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32, traveled: &mut Vec<i32>, actual_sum: i32, result: i32) -> i32 {
    let mut result = result;
    let tree_node = node.as_ref().unwrap().as_ref().borrow();
    traveled.push(tree_node.val);
    println!("{:?}", traveled);
    if actual_sum + tree_node.val == sum {
        println!("Ok");
        result += 1;
    }
    let mut traveled_copy = traveled.clone();
    let mut actual_sum_copy = actual_sum + tree_node.val;
    actual_sum_copy -= traveled_copy.remove(0);
    while  actual_sum_copy != sum && traveled_copy.len() > 0 {
        actual_sum_copy -= traveled_copy.remove(0);
    }
    if actual_sum_copy == sum && traveled_copy.len() > 0 {
        println!("Ok1");
        println!("c: {:?}", traveled_copy);

        result += 1;
        println!("{:?}", traveled);
    }

    if tree_node.left != None{
        result = pre_order_travel(&tree_node.left, sum, traveled, actual_sum + tree_node.val, result);
    }
    if tree_node.right != None {
        result = pre_order_travel(&tree_node.right, sum, traveled, actual_sum + tree_node.val, result);
    }
    traveled.pop();
    result
}


pub fn create_tree(a: &mut Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if a.len() == 0 {
        return None;
    }
    let x = a.remove(0);
    if x == None {
        return None;
    } else {
        let mut tree = TreeNode::new(x.unwrap());
        tree.left = create_tree(a);
        tree.right = create_tree(a);
        Some(Rc::new(RefCell::new(tree)))
    }
}