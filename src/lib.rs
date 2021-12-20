use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn tree_node_wrap(node: TreeNode) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn new_left_right(val: i32, left: i32, right: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: Self::tree_node_wrap(Self::new(right)),
        }
    }

    pub fn new_left(val: i32, left: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: None,
        }
    }

    pub fn new_right(val: i32, right: i32) -> Self {
        let right = Self::new(right);
        TreeNode {
            val,
            left: None,
            right: Some(Rc::new(RefCell::new(right))),
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn test_fixture_1() -> Vec<i32> {
        vec![3, 2, 1, 6, 0, 5]
    }

    pub fn test_fixture_2() -> Vec<i32> {
        vec![3, 2, 1]
    }

    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }

        let (val, left, right) = Solution::split(&nums);

        let tree;

        if let Some(node_value) = val {
            let mut node = TreeNode::new(node_value);
            node.left = Solution::construct_maximum_binary_tree(left);
            node.right = Solution::construct_maximum_binary_tree(right);
            tree = TreeNode::tree_node_wrap(node);
        } else {
            tree = None;
        }

        tree
    }

    /*
      - find the index of the max value
      - split the vec into two vecs:
        left and right
    */
    pub fn split(nums: &Vec<i32>) -> (Option<i32>, Vec<i32>, Vec<i32>) {
        if nums.len() == 0 {
            return (None, vec![], vec![]);
        }

        let mut max: (usize, &i32) = (0, &nums[0]);
        for (index, val) in nums.iter().enumerate() {
            if max.1 <= val {
                max = (index, val);
            }
        }

        /*
          - the max val is included in the right vec
        */
        let (left, val_right) = nums.split_at(max.0);

        let right_vec;

        /*
          - exclude the max val from the right vec
        */
        if let Some((_, elements)) = val_right.split_first() {
            right_vec = elements.to_vec()
        } else {
            right_vec = vec![];
        }

        (Some(*max.1), left.to_vec(), right_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let lr = TreeNode::new_right(2, 1);
        let mut l = TreeNode::new(3);
        l.right = TreeNode::tree_node_wrap(lr);
        let r = TreeNode::new_left(5, 0);
        let mut root = TreeNode::new(6);
        root.left = TreeNode::tree_node_wrap(l);
        root.right = TreeNode::tree_node_wrap(r);
        let target = TreeNode::tree_node_wrap(root);
        let result = Solution::construct_maximum_binary_tree(Solution::test_fixture_1());
        assert_eq!(result, target);
    }

    #[test]
    fn sample_2() {
        let r = TreeNode::new_right(2, 1);
        let mut root = TreeNode::new(3);
        root.right = TreeNode::tree_node_wrap(r);
        let target = TreeNode::tree_node_wrap(root);
        let result = Solution::construct_maximum_binary_tree(Solution::test_fixture_2());
        assert_eq!(result, target);
    }
}
