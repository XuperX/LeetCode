use super::super::data_structures::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut target = &mut result;

        'k: while let Some(mut head_node) = head {
            // Find a group that has the length of k.

            let mut length = 1;
            let mut node = head_node.as_mut();

            while length < k {
                if let Some(next) = node.next.as_mut() {
                    length += 1;
                    node = next;
                } else {
                    *target = Some(head_node);

                    break 'k;
                }
            }

            head = node.next.take();

            // Reverse the group.

            let mut next = head_node.next.take();
            let mut group = head_node;

            while let Some(mut next_node) = next {
                next = next_node.next.replace(group);
                group = next_node;
            }

            *target = Some(group);

            // Find the next target.

            let mut node = target;

            while let Some(node_2) = node {
                node = &mut node_2.next;
            }

            target = node;
        }

        result
    }
}

impl super::Solution for Solution {
    fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        Self::reverse_k_group(head, k)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
