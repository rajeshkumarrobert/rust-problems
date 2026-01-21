use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct KthLargest {
    heap: BinaryHeap<i32>,
    k: usize,
}

impl KthLargest {

    pub fn new(k: usize, nums: Vec<i32>) -> Self {
        KthLargest {
            k,
            heap: BinaryHeap::from_iter(nums.iter().map(|&n| -n)),
        }
    }
    
    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(-val);
        while self.heap.len() > self.k as usize {
            self.heap.pop();
        }
        -self.heap.peek().unwrap()
    }
}

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut res = BinaryHeap::new();
        for x in stones{
            res.push(x);
        }
        while res.len()!=1 {
            if res.len() == 0{
                break;
            }
            let x = res.pop().unwrap_or(0);
            let y = res.pop().unwrap_or(0);
            let val = x-y;
            match val{
                0 => (),
                _ => res.push(x-y),
            }
        }
        let result = match res.pop() {
            Some(val) => val,
            None => 0
        };
        result
    }