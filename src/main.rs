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
    integer_operations::high_and_low,
    opposite::opposite,
    series_sum::series_sum,
    exes_and_ohs::xo,
};
use leetcode::{merge_alternately,merge_sorted_array,three_sum,remove_duplicates};
use dsa::{bubble_sorting::bubble_sort, selection_sorting::selection_sort,
    insertion_sorting::insertion_sort};
use leetcode::add_two_numbers::{ListNode, Solution};
mod codewars;
mod leetcode;
mod dsa;
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
    println!("{}",opposite(23));
    println!("{}",series_sum(3));
    println!("{}",merge_alternately::merge_alternately("ab".to_string(), "pqrs".to_string()));
    println!("{}",merge_alternately::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string()));
    merge_sorted_array::merge(vec![1,2,3,0,0,0], 3, vec![2,5,6], 3);
    merge_sorted_array::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2);
    println!("{}",xo("xo"));
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
    println!("Bubble sort: {:?}",bubble_sort(vec![5,9,2,4,3]));
    println!("Selection sort: {:?}",selection_sort(vec![5,9,2,4,3]));
    println!("Insertion sort: {:?}",insertion_sort(vec![5,9,2,4,3]));
    //println!("The three sum result is {:?}",three_sum::three_sum(vec![])); //Need to work
    println!("Remove duplicates: {:?}",remove_duplicates::remove_duplicates(&mut vec![1,1,2,3]));
    println!("Search insert:{:?}",merge_sorted_array::search_insert(vec![1,3,5,6], 7));
    println!("plus one:{:?}",merge_sorted_array::plus_one(vec![1,2,3]))
}
