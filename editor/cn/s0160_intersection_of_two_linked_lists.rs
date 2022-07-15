struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

//There is no code of Rust type for this problem
impl Solution {
    pub fn getIntersectionNode(
        h1: Option<Box<ListNode>>,
        h2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut next_h1 = h1.clone();
        let mut next_h2 = h2.clone();
        while next_h1.as_ref() != next_h2.as_ref() {
            next_h1 = match next_h1 {
                Some(node) => node.next,
                None => h2.clone(),
            };
            next_h2 = match next_h2 {
                Some(node) => node.next,
                None => h1.clone(),
            }
        }

        next_h1
    }
}

pub fn build_list_node(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut();
    for i in v {
        let next = ListNode::new(*i);
        let v = current.unwrap();
        //        let Some(v) = current;
        v.next = Some(Box::new(next));
        current = v.next.as_mut();
    }
    //    println!("head={:#?}",head);
    head.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rustgym_util::*;

    #[test]
    fn test() {
        let h1 = build_list_node(&vec![1, 0, 1, 2, 3, 4, 5]);
        let h2 = build_list_node(&vec![7, 3, 4, 5]);
        println!(
            "{:?}",
            Solution::getIntersectionNode(h1.clone(), h2.clone())
        );
        assert_eq!(
            Solution::getIntersectionNode(h1, h2),
            build_list_node(&vec![3, 4, 5])
        );
    }
}
