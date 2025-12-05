use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::leetcode::breadth_first_search::TreeNode;

pub struct MyStack {
    data:Vec<i32>
}

impl MyStack {

    pub fn new() -> Self {
        MyStack{
            data: Vec::new()
        }
    }
    
    pub fn push(&mut self, x: i32) {
        self.data.push(x);
    }
    
    pub fn pop(&mut self) -> i32 {
        match self.data.pop(){
            Some(x) => return x,
            None => 0
        }
    }
    
    pub fn top(&mut self) -> i32 {
        let last_index = self.data.len()-1;
        self.data[last_index]
    }
    
    pub fn empty(&self) -> bool {
        self.data.len() == 0
    }
}

pub struct MyQueue {
 data: VecDeque<i32>
}

impl MyQueue {

    pub fn new() -> Self {
        MyQueue { data: VecDeque::new() }
    }
    
    pub fn push(&mut self, x: i32) {
        self.data.push_back(x);
    }
    
    pub fn pop(&mut self) -> i32 {
        self.data.pop_front().unwrap_or(0)
    }
    
    pub fn peek(&self) -> i32 {
        self.data[0]
    }
    
    pub fn empty(&self) -> bool {
        self.data.len() == 0
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
}
 
impl ListNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }

 pub fn build_node_from_array(nums: &[Option<i32>]) -> Option<Box<ListNode>> {
        if nums.is_empty() || nums[0].is_none() {
            return None;
        }

        // find first Some value
        let mut idx = 0;
        while idx < nums.len() && nums[idx].is_none() {
            idx += 1;
        }
        if idx == nums.len() {
            return None;
        }

        // create head from first Some(...)
        let mut head = Box::new(ListNode::new(nums[idx].unwrap()));
        let mut current = &mut head;

        // append subsequent Some(...) values, skip None entries
        for i in (idx + 1)..nums.len() {
            if let Some(val) = nums[i] {
                let node = Box::new(ListNode::new(val));
                current.next = Some(node);
                current = current.next.as_mut().unwrap();
            }
        }

        Some(head)
    
    }
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut res = vec![];
        fn dfs(node:Option<Box<ListNode>>, res:&mut Vec<i32>){
            match node {
                Some(val) =>{
                    res.push(val.val);
                    dfs(val.next, res);
                },
                None => (),
            }
        }
        dfs(head, &mut res);
        let original = res.clone();
        res.reverse();
        //res == res.iter().rev().cloned().collect::<Vec<_>>()
        original == res
    }

pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut res: Vec<i32> = Vec::new();
    for x in operations {
        match x.as_str() {
            "+" => {
                if res.len() >= 2 {
                    let l = res.len();
                    let sum = res[l - 1] + res[l - 2];
                    res.push(sum);
                }
            }
            "D" => {
                if let Some(&last) = res.last() {
                    res.push(last * 2);
                }
            }
            "C" => {
                res.pop();
            }
            other => {
                if let Ok(n) = other.parse::<i32>() {
                    res.push(n);
                }
            }
        }
    }
    res.iter().sum()
}

pub fn backspace_compare(s: String, t: String) -> bool {
        let mut vec_s = vec![];
        let mut vec_t = vec![];
        for s in s.chars() {
            match s {
                '#' => {vec_s.pop();
                    ()},
                _ => vec_s.push(s),
            }
        }
        for t in t.chars() {
            match t {
                '#' => {vec_t.pop();
                ()},
                _ => vec_t.push(t),
            }
        }
        vec_s == vec_t
    }

pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>)->Option<Rc<RefCell<TreeNode>>> {
        let mut res = vec![];
        fn change_bfs(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32> ){
            match node {
                Some(root) =>{
                    let left = root.borrow().left.clone();
                    let right = root.borrow().right.clone();
                    change_bfs(left, arr);
                    arr.push(root.borrow().val);
                    change_bfs(right, arr);
                },
                None => ()
            }
        }
        change_bfs(root, &mut res);
        println!("the bfs:{:?}", res);
        let root = Rc::new(RefCell::new(TreeNode::new(*res.get(0).unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while i < res.len() {
            if let Some(node) = queue.pop_front() {
                // Left child
                if i < res.len() {
                    node.borrow_mut().left = None;    
                }

                // Right child
                if i < res.len() {
                    if let Some(val) = res.get(i) {
                        let right = Rc::new(RefCell::new(TreeNode::new(*val)));
                        node.borrow_mut().right = Some(right.clone());
                        queue.push_back(right);
                    }
                    i += 1;
                }
            }
        }

        Some(root)
    }

pub fn remove_outer_parentheses(s: String) -> String {
        let s_chars = s.chars().collect::<Vec<_>>();
        let mut vec_que = VecDeque::new();
        let mut res: Vec<String> = vec![];
        for ch in s_chars {
            let open_parantensis = vec_que.iter().filter(|x| **x == '(').count();
            let close_parantensis = vec_que.iter().filter(|x| **x == ')').count();
            if open_parantensis != close_parantensis || open_parantensis == 0 || close_parantensis == 0  {
                vec_que.push_back(ch);
            } else {
                // remove outer parentheses and capture inner content
                if !vec_que.is_empty() {
                    vec_que.pop_front();
                }
                if !vec_que.is_empty() {
                    vec_que.pop_back();
                }
                res.push(vec_que.iter().cloned().collect::<String>());
                vec_que.clear();
                vec_que.push_front(ch);
            }
        }
        // remove outer parentheses and capture inner content
        if !vec_que.is_empty() {
            vec_que.pop_front();
        }
        if !vec_que.is_empty() {
            vec_que.pop_back();
        }
        res.push(vec_que.iter().cloned().collect::<String>());
        vec_que.clear();
        res.into_iter().collect::<String>()
    }

pub fn remove_duplicates(s: String) -> String {
    let mut queue = VecDeque::new();
        for x in s.chars(){
            let temp = queue.back();
            match temp {
                Some(value) =>{
                    if *value == x{
                          queue.pop_back(); 
                    }else {
                         queue.push_back(x);
                    }
                },
                None => queue.push_back(x),
            }
        }
        queue.iter().collect::<String>()
    }

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    for (index, value) in prices.iter().enumerate() {
        let start = index + 1;
        let mut found = false;
        for i in start..prices.len() {
            if prices[i] <= *value {
                res.push(value - prices[i]);
                found = true;
                break;
            }
        }
        if !found {
            res.push(*value);
        }
    }
    res
}

pub fn make_good(s: String) -> String {
        let mut queue:VecDeque<char> = VecDeque::new();
        for c in s.chars(){
            let temp = queue.back();
            match temp {
                Some(value) =>{
                    if (value.is_ascii_uppercase() && (value.to_ascii_lowercase() == c)) ||
                        (value.is_ascii_lowercase() && (value.to_ascii_uppercase() == c)){
                          queue.pop_back(); 
                    }else {
                         queue.push_back(c);
                    }
                },
                None => queue.push_back(c),
            }
        }
        queue.iter().collect::<String>()
    }

//medium
pub fn simplify_path(path: String) -> String {
        let mut simplified_path = vec![];
        for dir in path.split('/') {
            match dir {
                "" | "." => continue,
                ".." => { simplified_path.pop(); }
                _ => simplified_path.push(dir),
            }
        }

        "/".to_owned() + &simplified_path.join("/")
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
            let mut queue = VecDeque::new();
            let mut node = head.take();
            while let Some(mut n) = node {
                node = n.next.take();
                queue.push_back(n);
            }
            let mut new = ListNode::new(0);
            let (mut ptr, mut front) = (&mut new, true);
            while !queue.is_empty() {
                ptr.next = match front {
                    true  => queue.pop_front(),
                    false => queue.pop_back(),
                };
                front = !front;
                ptr = ptr.next.as_mut().unwrap();
            }
            *head = new.next;
    }

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut res: Vec<i32> = Vec::new();
        for x in tokens {
            match x.as_str() {
                "+" | "-" | "*" | "/" => {
                    let a = res.pop().unwrap();
                    let b = res.pop().unwrap();
                    let result = match x.as_str() {
                        "+" => b + a,
                        "-" => b - a,
                        "*" => b * a,
                        "/" => b / a,
                        _ => unreachable!(),
                    };
                    res.push(result);
                },
                _ => res.push(x.parse::<i32>().unwrap()),
            }
        }
        res[0]
    }
#[derive(Debug)]
pub struct MinStack {
    pub st: Vec<(i32, i32)>,
}

impl MinStack {

    pub fn new() -> Self {
        Self { st: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        if let Some(last) = self.st.last() {
            self.st.push((val, val.min(last.1)));
        } else {
            self.st.push((val, val));
        }
    }

    pub fn pop(&mut self) {
        self.st.pop();
    }

    pub fn top(&self) -> i32 {
        self.st.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.st.last().unwrap().1
    }
}