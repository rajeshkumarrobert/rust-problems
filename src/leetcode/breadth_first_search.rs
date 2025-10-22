use std::{cell::RefCell, rc::Rc};

//Definition for a binary tree node.
#[derive(Clone,Debug, PartialEq, Eq)]
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
      right: None
    }
  }
  pub fn build_tree_from_array(nums: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() || nums[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < nums.len() {
        if let Some(node) = queue.pop_front() {
            // Left child
            if i < nums.len() {
                if let Some(val) = nums[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }

            // Right child
            if i < nums.len() {
                if let Some(val) = nums[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }
    }

    Some(root)
}

}
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
        
        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                traversal(n.borrow().left.clone(), res);
                res.push(n.borrow().val);
                traversal(n.borrow().right.clone(), res);
            }
        }
        
        traversal(root, &mut res);
        
        res
}

pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p_node), Some(q_node)) => {
                let p_borrow = p_node.borrow();
                let q_borrow = q_node.borrow();

                p_borrow.val == q_borrow.val
                    && is_same_tree(p_borrow.left.clone(), q_borrow.left.clone())
                    && is_same_tree(p_borrow.right.clone(), q_borrow.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
       match root {
            Some(node) => {
                let left = Option::clone(&node.borrow().left);
                let right = Option::clone(&node.borrow().right);
                helper(left, right)
            }
            None => true,
        }
    }

    fn helper(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p_node), Some(q_node)) => {
                let p_borrow = p_node.borrow();
                let q_borrow = q_node.borrow();

                p_borrow.val == q_borrow.val
                    && helper(p_borrow.left.clone(), q_borrow.right.clone())
                    && helper(p_borrow.right.clone(), q_borrow.left.clone())
            }
            (None, None) => true,
            _ => false,
        } 
    }

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
       match root {
            Some(rc_node) => {
                let l = Option::clone(&rc_node.borrow().left);
                let r = Option::clone(&rc_node.borrow().right);
                1 + max_depth(l).max(max_depth(r))
            }
            None => 0,
       } 
    }

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(root) => {
                    let left = dfs(root.borrow().left.clone());
                    let right = dfs(root.borrow().right.clone());
                    if (left-right).abs() > 1 || left == -1 || right == -1 {
                        return -1
                    }
                    left.max(right) + 1
                }
                None => 0
            }
        }
        dfs(root) != -1
    }

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = min_depth(node.borrow().left.clone());
                let right = min_depth(node.borrow().right.clone());

                if left == 0 || right == 0 {
                    return left.max(right) + 1
                }
                
                left.min(right) + 1
            },
            None => 0
        }
    }

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        find_sum(root, target_sum, 0)
    }

    fn find_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, current_sum: i32) -> bool {
        if let Some(node) = root {
            let sum = current_sum + node.borrow().val;
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            match (left, right) {
                (None, None) => target_sum == sum,
                (l, r) => {
                    find_sum(l, target_sum, sum) || find_sum(r, target_sum, sum)
                }
            }
        } else {
            false
        }
    }

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        
        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                res.push(n.borrow().val);
                traversal(n.borrow().left.clone(), res);
                traversal(n.borrow().right.clone(), res);
            }
        }
        
        traversal(root, &mut res);
        
        res
    }

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                traversal(n.borrow().left.clone(), res);
                traversal(n.borrow().right.clone(), res);
                res.push(n.borrow().val);
            }
        }
        traversal(root, &mut res);
        res
    }