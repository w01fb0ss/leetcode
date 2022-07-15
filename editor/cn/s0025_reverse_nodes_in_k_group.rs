struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut next_head = &mut head;
        for _ in 0..k {
            next_head = match next_head.as_mut() {
                Some(node) => &mut node.next,
                None => return head,
            };
        }
        let next_head = Self::reverse_k_group(next_head.take(), k);
        Self::reverse(head, next_head)

    }

    fn reverse(mut head: Option<Box<ListNode>>, mut next_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        while let Some(mut node) = head {
            head = node.next.take();
            // link head -> next_head
            node.next = next_head.take();
            // as tail
            next_head = Some(node);
        }
        next_head
    }



}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::*;
    // use rustgym_util::*;

    #[test]
    fn test() {
        
    }
}