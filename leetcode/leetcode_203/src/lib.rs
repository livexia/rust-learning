use std::borrow::Borrow;

/**
给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。
 
示例 1：
输入：head = [1,2,6,3,4,5,6], val = 6
输出：[1,2,3,4,5]

示例 2：
输入：head = [], val = 1
输出：[]

示例 3：
输入：head = [7,7,7,7], val = 7
输出：[]

提示：

列表中的节点在范围 [0, 104] 内
1 <= Node.val <= 50
0 <= k <= 50


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/remove-linked-list-elements
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy_node = Some(Box::new(ListNode{ val: 0, next: head }));
    let mut p = &mut dummy_node;
    while let Some(node) = &mut p.as_mut().unwrap().next {
        if node.val == val {
            p.as_mut().unwrap().next = node.next.take();
        } else {
            p = &mut p.as_mut().unwrap().next;
        }
    }
    dummy_node.unwrap().next
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
