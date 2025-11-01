use std::{cell::RefCell, collections::HashMap, rc::Rc};

//Definition for a binary tree node.
#[derive(Clone, Debug, PartialEq, Eq)]
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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
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
                if (left - right).abs() > 1 || left == -1 || right == -1 {
                    return -1;
                }
                left.max(right) + 1
            }
            None => 0,
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
                return left.max(right) + 1;
            }

            left.min(right) + 1
        }
        None => 0,
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
            (l, r) => find_sum(l, target_sum, sum) || find_sum(r, target_sum, sum),
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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let left = invert_tree(node.borrow_mut().left.clone());
            let right = invert_tree(node.borrow_mut().right.clone());
            node.borrow_mut().right = left;
            node.borrow_mut().left = right;
            Some(node)
        }
        None => None,
    }
}

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut temp: Vec<i32> = vec![];
    fn traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<String>, temp: &mut Vec<i32>) {
        if let Some(n) = node {
            let value = n.borrow().val;
            temp.push(value);
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();
            if left.is_none() && right.is_none() {
                // leaf node: build path string from temp
                let path = temp
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("->");
                res.push(path);
            } else {
                traversal(left, res, temp);
                traversal(right, res, temp);
            }
            temp.pop();
        }
    }
    traversal(root, &mut res, &mut temp);
    res
}

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut arr = Vec::new();
    fn collect_left_leaf(tree: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>, is_left: bool) {
        if let Some(node) = tree {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            if node.borrow().right.is_none() {
                let left_tree = node.borrow().left.clone();
                collect_left_leaf(left_tree, arr, true);
                if node.borrow().left.is_none() && node.borrow().right.is_none() && is_left {
                    arr.push(node.borrow().val);
                }
            } else {
                collect_left_leaf(left, arr, true);
                collect_left_leaf(right, arr, false);
            }
        }
    }
    collect_left_leaf(root, &mut arr, false);
    arr.iter().sum::<i32>()
}

pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    fn collect_mode(node: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) {
        if let Some(tree) = node {
            let left = tree.borrow().left.clone();
            let right = tree.borrow().right.clone();
            let value = tree.borrow().val;
            let counter = map.entry(value).or_insert(0);
            *counter += 1;
            collect_mode(left, map);
            collect_mode(right, map);
        }
    }
    collect_mode(root, &mut map);
    let mut list = map.iter().collect::<Vec<(&i32, &i32)>>();
    list.sort_unstable_by(|a, b| b.1.cmp(a.1));
    let (mut res, prev) = (vec![*list[0].0], list[0].1);
    for i in 1..list.len() {
        if prev != list[i].1 {
            break;
        }
        res.push(*(list[i].0));
    }
    res
}

pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = Vec::new();
    fn node_difference(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(tree) = node {
            let left = tree.borrow().left.clone();
            let right = tree.borrow().right.clone();
            node_difference(left, res);
            res.push(tree.borrow().val);
            node_difference(right, res);
        }
    }
    node_difference(root, &mut res);
    res.windows(2).map(|v| (v[1] - v[0]).abs()).min().unwrap()
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, low: i64, high: i64) -> bool {
        match node {
            None => true,
            Some(n) => {
                let v = n.borrow().val as i64;
                if v <= low || v >= high {
                    false
                } else {
                    dfs(&n.borrow().left, low, v) && dfs(&n.borrow().right, v, high)
                }
            }
        }
    }

    dfs(&root, i64::MIN, i64::MAX)
}

pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = vec![];
    let mut curr = root.clone();
    let mut x = None;
    let mut y = None;
    let mut pred: Option<Rc<RefCell<TreeNode>>> = None;

    while !(stack.is_empty() && curr.is_none()) {
        while let Some(node) = curr {
            curr = node.borrow_mut().left.clone();
            stack.push(node);
        }
        if let Some(node) = stack.pop() {
            if let Some(p) = pred {
                if p.borrow_mut().val > node.borrow_mut().val {
                    y = Some(node.clone());
                    if x.is_none() {
                        x = Some(p);
                    } else {
                        break;
                    }
                }
            }
            pred = Some(node.clone());
            curr = node.borrow_mut().right.clone();
        }
    }
    let (x_opt, y_opt) = (x, y);
    if let (Some(xn), Some(yn)) = (x_opt, y_opt) {
        let mut xb = xn.borrow_mut();
        let mut yb = yn.borrow_mut();
        std::mem::swap(&mut xb.val, &mut yb.val);
    }
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();
    let word = word.chars().collect::<Vec<_>>();
    let len = word.len();
    let mut visited = vec![vec![false; n]; m];

    fn dfs(
        board: &[Vec<char>],
        word: &[char],
        visited: &mut [Vec<bool>],
        m: usize,
        n: usize,
        len: usize,
        i: usize,
        j: usize,
        depth: usize,
    ) -> bool {
        depth == len
            || (i < m && j < n && !visited[i][j] && word[depth] == board[i][j] && {
                visited[i][j] = true;
                let rez = [0, 1, 0, !0, 0].windows(2).any(|w| {
                    dfs(
                        board,
                        word,
                        visited,
                        m,
                        n,
                        len,
                        i.wrapping_add(w[0]),
                        j.wrapping_add(w[1]),
                        depth + 1,
                    )
                });
                visited[i][j] = false;
                rez
            })
    }
    (0..m).any(|i| (0..n).any(|j| dfs(&board, &word, &mut visited, m, n, len, i, j, 0)))
}

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();

        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            target: i32,
            curr_sum: i32,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if let Some(n) = node {
                let val = n.borrow().val;
                path.push(val);
                let new_sum = curr_sum + val;
                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();

                if left.is_none() && right.is_none() {
                    if new_sum == target {
                        res.push(path.clone());
                    }
                } else {
                    dfs(left, target, new_sum, path, res);
                    dfs(right, target, new_sum, path, res);
                }
                path.pop();
            }
        }

        dfs(root, target_sum, 0, &mut path, &mut res);
        res
    }

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    pub fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_borrowed = node.borrow_mut();
            let mut left = node_borrowed.left.take();
            let mut right = node_borrowed.right.take();

            if left.is_none() && right.is_none() {
                Some(node.clone())
            } else {
                let left_flatten_tail = helper(&mut left);
                let right_flatten_tail = helper(&mut right);
                node_borrowed.left = None;
                node_borrowed.right = right.clone();

                if left.is_some() {
                    node_borrowed.right = left;
                }
                if let Some(lft) = left_flatten_tail.clone() {
                    lft.borrow_mut().right = right;
                }

                right_flatten_tail
                    .or(left_flatten_tail)
                    .or(Some(node.clone()))
            }
        } else {
            None
        }
        
    }
    helper(root);
}