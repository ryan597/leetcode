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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut head: Box<ListNode>;
        let mut tmp1 = list1;
        let mut tmp2 = list2;

        match (tmp1.as_ref(), tmp2.as_ref()) {
            (Some(n1), Some(n2)) => {
                if (n1.val <= n2.val) {
                    head = n1.clone();
                    tmp1 = tmp1.unwrap().next;
                }
                else {
                    head = n2.clone();
                    tmp2 = tmp2.unwrap().next;
                }
            },
            (None, None) => return None,
            (None, Some(n2)) => return tmp2,
            (Some(n1), None) => return tmp1,
        };

        let mut node = head.as_mut();

        loop {
            match (tmp1.as_ref(), tmp2.as_ref()) {
                (Some(n1), Some(n2)) => {
                    if n1.val <= n2.val {
                        node.next = tmp1.clone();
                        tmp1 = tmp1.unwrap().next;
                    }
                    else {
                        node.next = tmp2.clone();
                        tmp2 = tmp2.unwrap().next;
                    }
                },
                (None, None) => return Some(head),
                (None, Some(n2)) => {
                    node.next = tmp2;
                    return Some(head)
                },
                (Some(n1), None) => {
                    node.next = tmp1;
                    return Some(head)
                },
            };
            node = node.next.as_mut().unwrap().as_mut();
        }
    }
}
