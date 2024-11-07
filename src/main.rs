use codewars::{count_positive_sum_negative::count_positives_sum_negatives, 
    make_negative::make_negative, 
    maps, 
    paperwork::paperwork, 
    reverse_array::digitize,
    greet_string::greet,
    messi_goals::goals,
    remove_char::remove_char,
    square_array::square_sum,
    reverse_string::solution,
    remove_vowel::disemvowel,
    integer_operations::high_and_low
};

use leetcode::add_two_numbers::{ListNode, Solution};
mod codewars;
mod leetcode;
fn main() {
    //println!("Hello, world!");
    digitize(348597);
    count_positives_sum_negatives(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15]);
    println!("{}",paperwork(5,-5 ));
    println!("{:?}",maps::maps(&vec![1, 2, 3, 4]));
    println!("{}",make_negative(1));
    println!("{}",greet("Ryan"));
    println!("{}",goals(43, 10, 5));
    println!("{}",remove_char("country"));
    println!("{}",square_sum(vec![5, 3, 4]));
    println!("{}",solution("world"));
    println!("{}",disemvowel("This website is for losers LOL!"));
    println!("{}",high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    // let mut l1 = ListNode::new(2 );
    // let mut second = ListNode::new(4);
    // let third = ListNode::new(3);
    // second.next=Some(Box::new(third));
    // l1.next=Some(Box::new(second));
    // let mut l2 = ListNode::new(5);
    // let mut second_l2 = ListNode::new(6);
    // let third_l2 = ListNode::new(4);
    // second_l2.next=Some(Box::new(third_l2));
    // l2.next = Some(Box::new(second_l2));
    // println!("{:?}",l1);
    // println!("{:?}",l2);
    // println!("{:?}",Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))))
}
