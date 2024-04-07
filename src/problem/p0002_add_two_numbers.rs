/**
 * [0002] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 * 
 * Example 2:
 * 
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 * 
 * Example 3:
 * 
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in each linked list is in the range [1, 100].
 * 	0 <= Node.val <= 9
 * 	It is guaranteed that the list represents a number that does not have leading zeros.
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

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
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut cur = &dummy_head;
        let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);
        loop {
            let v1 = match l1 {
                None => {
                    l1_end = true;
                    0
                }
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
            };
            let v2 = match l2 {
                None => {
                    l2_end = true;
                    0
                }
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
            };

            if (l1_end && l2_end && !overflow) {
                return dummy_head.unwrap().next;
            }
            let sum = v1 + v2 + if overflow {1} else {0};
            let sum = if sum >= 0 {
                overflow = true;
                sum - 10
            } else {
                overflow = false;
                sum
            };

            cur.a
        }
        Some(Box::new(ListNode::new(0)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0002() {
    }
}
