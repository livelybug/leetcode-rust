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

fn loopList(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut head_prt: &mut Option<Box<ListNode>>, mut carry: i32) {

    let mut l1_next = None;
    let mut l2_next = None;
    let mut node1_val = 0;
    let mut node2_val = 0;

    if let Some(node1) = l1 {
        node1_val = node1.val;
        // println!("{:?}", node1.val);
        l1_next = node1.next;
    } 

    if let Some(mut node2) = l2 {
        node2_val = node2.val;
        // println!("{:?}", node2.val);
        l2_next = node2.next;
    }

    let mut sum = node1_val + node2_val + carry;
    if(sum > 9) {
        sum = sum % 10;
        carry = 1;
    } else {
        carry = 0;
    }

    *head_prt = Some(Box::new(ListNode::new(sum)));
    head_prt = &mut head_prt.as_mut().unwrap().next;

    if(l1_next != None || l2_next != None || carry != 0) {
        loopList(l1_next, l2_next, head_prt, carry)
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        loopList(l1, l2, &mut head, 0);

        head
    }
}
