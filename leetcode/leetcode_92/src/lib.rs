/**
反转从位置 m 到 n 的链表。请使用一趟扫描完成反转。

说明:
1 ≤ m ≤ n ≤ 链表长度。

示例:

输入: 1->2->3->4->5->NULL, m = 2, n = 4
输出: 1->4->3->2->5->NULL


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/reverse-linked-list-ii
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

pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut phead = Some(Box::new(ListNode::new(0)));
    phead.as_mut().unwrap().next = head;
    let mut p = &mut phead;
    for _ in 0..left - 1 {
        p = &mut p.as_mut().unwrap().next;
    }
    let mut mhead = p.as_mut().unwrap().next.take();
    let mut q = &mut mhead;
    for _ in left..right {
        q = &mut q.as_mut().unwrap().next;
    }
    let nnext = q.as_mut().unwrap().next.take();
    let out = reverse_list(mhead);
    p.as_mut().unwrap().next = out;
    while p.as_mut().unwrap().next.is_some() {
        p = &mut p.as_mut().unwrap().next;
    }
    p.as_mut().unwrap().next = nnext;


    phead.unwrap().next
}

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pnode = None;
    while  let Some(mut node) = head {
        let pnext = node.next;
        node.next = pnode;
        pnode = Some(node);
        head = pnext;
    }
    pnode
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
