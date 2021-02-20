/**
给你一个链表数组，每个链表都已经按升序排列。

请你将所有链表合并到一个升序链表中，返回合并后的链表。

示例 1：
输入：lists = [[1,4,5],[1,3,4],[2,6]]
输出：[1,1,2,3,4,4,5,6]
解释：链表数组如下：
[
  1->4->5,
  1->3->4,
  2->6
]
将它们合并到一个有序链表中得到。
1->1->2->3->4->4->5->6

示例 2：
输入：lists = []
输出：[]

示例 3：
输入：lists = [[]]
输出：[]
 
提示：
k == lists.length
0 <= k <= 10^4
0 <= lists[i].length <= 500
-10^4 <= lists[i][j] <= 10^4
lists[i] 按 升序 排列
lists[i].length 的总和不超过 10^4

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/merge-k-sorted-lists
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

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let l  = lists.len();
    if l == 0 { return None; }
    merge(&mut lists, 0, l - 1)
}

pub fn merge(lists: &mut Vec<Option<Box<ListNode>>>, l: usize, r: usize) -> Option<Box<ListNode>> {
    if l == r { return lists[l].take(); }
    let mid = (l + r) /2;
    return merge_two_lists(merge(lists, l, mid), merge(lists, mid + 1, r));
}

pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut tail = &mut head;

    while list1.is_some() && list2.is_some() {
        let (p1, p2) = (list1.as_deref_mut().unwrap(), list2.as_deref_mut().unwrap());
        if p1.val < p2.val {
            let next = p1.next.take();
            tail.next = list1.take();
            list1 = next;
        } else {
            let next = p2.next.take();
            tail.next = list2.take();
            list2 = next;
        }
        tail = tail.next.as_deref_mut().unwrap();
    }
    tail.next = list1.or(list2);
    head.next
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
