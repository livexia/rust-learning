/**
给你一个链表的头节点 head ，旋转链表，将链表每个节点向右移动 k 个位置。

示例 1：
输入：head = [1,2,3,4,5], k = 2
输出：[4,5,1,2,3]

示例 2：
输入：head = [0,1,2], k = 4
输出：[2,0,1]

提示：
链表中节点的数目在范围 [0, 500] 内
-100 <= Node.val <= 100
0 <= k <= 2 * 109

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/rotate-list
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

pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    debug_assert!(k >= 0);
    if head.is_none() || k == 0 {
        return head;
    }
    let mut len = 0;
    let mut ptr = &head;
    while let Some(ref node) = ptr {
        len += 1;
        ptr = &node.next;
    }

    let step = k % len;

    if step == 0 || len == 1 {
        return head;
    }
    let mut ptr = &mut head;
    for _ in 1..(len - step) {
        ptr = &mut ptr.as_mut().unwrap().next;
    }

    let mut new_head = ptr.as_mut().unwrap().next.take();
    let mut tail = &mut new_head;
    while tail.is_some() && tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    tail.as_mut().unwrap().next = head;

    new_head
}
    

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
