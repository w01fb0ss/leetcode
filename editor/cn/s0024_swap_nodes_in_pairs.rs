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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut first_node = head;
        let mut second_node = first_node.as_mut().unwrap().next.take();

        first_node.as_mut().unwrap().next = Self::swap_pairs(second_node.as_mut().unwrap().next.take());
        second_node.as_mut().unwrap().next = first_node;
        second_node
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