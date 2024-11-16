use super::ListNode;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let (mut current, mut carry, mut l1_ptr, mut l2_ptr) = (&mut head, 0, l1, l2);

        // * 遍历两个链表，逐位计算当前位置和，并与进位值相加
        // * 遍历结束后，如果还有进位值，则在链表末尾添加一个新的节点
        while l1_ptr.is_some() || l2_ptr.is_some() || carry != 0 {
            let sum = (l1_ptr.as_ref().map_or(0, |n| n.val))
                + (l2_ptr.as_ref().map_or(0, |n| n.val))
                + carry;

            l1_ptr = l1_ptr.and_then(|n| n.next);
            l2_ptr = l2_ptr.and_then(|n| n.next);

            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            current = &mut current.as_mut().unwrap().next;
            carry = sum / 10;
        }

        head.unwrap().next
    }
}
