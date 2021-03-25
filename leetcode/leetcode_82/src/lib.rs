/**
给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。

示例 1:
输入: 1->2->3->3->4->4->5
输出: 1->2->5

示例 2:
输入: 1->1->1->2->3
输出: 2->3

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list-ii
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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut root = ListNode { val: 0, next: head };
    let mut p = &mut root;

    while p.next.is_some() && p.next.as_ref().unwrap().next.is_some() {
        if p.next.as_ref().unwrap().val == p.next.as_ref().unwrap().next.as_ref().unwrap().val {
            let x = p.next.as_ref().unwrap().val;
            while p.next.is_some() && p.next.as_ref().unwrap().val == x {
                p.next = p.next.as_mut().unwrap().next.take();
            }
        } else {
            p = p.next.as_mut().unwrap();
        }
    }

    root.next
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
