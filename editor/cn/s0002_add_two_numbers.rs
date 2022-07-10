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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut flag = head.as_mut();
        let (mut p, mut q, mut carry) = (l1, l2, 0);
        while p.is_some() || q.is_some() {
            let mut sum = carry;
            if let Some(v) = p {
                sum += v.val;
                p = v.next;
            }
            if let Some(v) = q {
                sum += v.val;
                q = v.next;
            }
            carry = sum / 10;
            if let Some(v) = flag {
                v.next = Some(Box::new(ListNode::new(sum % 10)));
                flag = v.next.as_mut();
            }
        }
        if carry > 0 {
            if let Some(v) = flag {
                v.next = Some(Box::new(ListNode::new(carry)));
            }

        }
        head.unwrap().next
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