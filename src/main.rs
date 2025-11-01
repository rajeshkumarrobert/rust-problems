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
use leetcode::{merge_alternately,merge_sorted_array,remove_duplicates,string_related_probs,
    hash_table_problems,math_related_problems,sorting_related_problems, two_pointers, binary_search::Solution,
    prefix_sum,dynamic_programming,greedy,breadth_first_search};
use dsa::{bubble_sorting::bubble_sort, selection_sorting::selection_sort,
    insertion_sorting::insertion_sort};

use crate::leetcode::{ binary_search, breadth_first_search::TreeNode, merge_sorted_array::NumArray};
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
    println!("plus one:{:?}",merge_sorted_array::plus_one(vec![1,2,3]));
    println!("Max profit:{:?}",merge_sorted_array::max_profit(vec![2,4,1]));
    println!("Pascal Triangle:{:?}",merge_sorted_array::generate(5));
    println!("Pascal row:{:?}",merge_sorted_array::get_row(3));
    println!("Single Number:{:?}",merge_sorted_array::single_number(vec![2,2,1,2]));
    println!("Majority Element:{:?}",merge_sorted_array::majority_element(vec![2,2,1,1,1,1,3,2]));
    println!("Contains Duplicate:{:?}",merge_sorted_array::contains_duplicate(vec![2,2,1,1,1,1,3,2]));
    println!("Contains nearby Duplicate:{:?}",merge_sorted_array::contains_nearby_duplicate(vec![1,5,1,0],2));
    println!("Summary Ranges:{:?}",merge_sorted_array::summary_ranges(vec![0,2,3,4,6,8,9]));
    println!("Missing Number:{:?}",merge_sorted_array::missing_number(vec![3,0,1]));
    println!("Is Brackets are Valid :{:?}",string_related_probs::is_valid("()".to_string()));
    println!("Longest common prefix :{:?}",string_related_probs::longest_common_prefix(vec!["flower".to_string(),"fan".to_string(),"flight".to_string()]));
    let value = &mut vec![0,1,0,3,12];
    merge_sorted_array::move_zeroes(value);
    println!("Move Zeros:{:?}",value);
    let obj = NumArray::new(vec![-2,0,3,-5,2,-1]);
    let ret_1: i32 = obj.sum_range(2,5);
    println!("Num Array:{:?}",ret_1);
    println!("Intersection Number:{:?}",merge_sorted_array::intersection(vec![4,9,5], vec![9,4,9,8,4]));
    println!("Roman to integer :{:?}",string_related_probs::roman_to_int("III".to_string()));
    println!("Subset String :{:?}",string_related_probs::str_str("sadbutsad".to_string(),"sad".to_string()));
    println!("last word size :{:?}",string_related_probs::length_of_last_word("luffy is still joyboy".to_string()));
    println!("Is palindrome :{:?}",string_related_probs::is_palindrome("A man, a plan, a canal: Panama".to_string()));
    println!("Excel title :{:?}",string_related_probs::convert_to_title(23));
    println!("Excel title to number:{:?}",string_related_probs::title_to_number("ZY".to_string()));
    println!("Is isomorphic:{:?}",string_related_probs::is_isomorphic("egg".to_string(),"add".to_string()));
    println!("Is anagram:{:?}",string_related_probs::is_anagram("rat".to_string(),"car".to_string()));
    println!("Pattern Matching:{:?}",string_related_probs::word_pattern("abba".to_string(),"dog cat cat fish".to_string()));
    let mut string_value = vec!['h','a','n','n','a','h'];
    string_related_probs::reverse_string(&mut string_value);
    println!("Reverse a String:{string_value:?}");
    println!("Reverse a Vowel:{:?}",string_related_probs::reverse_vowels("a.".to_string()));
    println!("can construct:{:?}",string_related_probs::can_construct("a".to_string(),"b".to_string()));
    println!("First Unique char:{:?}", string_related_probs::first_uniq_char("aabb".to_string()));
    println!("Find the difference:{:?}", string_related_probs::find_the_difference("abcd".to_string(),"abcde".to_string()));
    println!("is Subsequence:{:?}", string_related_probs::is_subsequence("abc".to_string(),"ahbgdc".to_string()));
    println!("Longest Palindrome:{:?}", string_related_probs::longest_palindrome("a".to_string()));
    println!("Fizz buzz:{:?}", string_related_probs::fizz_buzz(3));
    //hashtable problems
    println!("Intersect:{:?}", hash_table_problems::intersect(vec![3,1,2],vec![1,1]));
    println!("Difference Number:{:?}", hash_table_problems::find_disappeared_numbers(vec![2,2]));
    println!("Next greater Element:{:?}", hash_table_problems::next_greater_element(vec![1,3,5,2,4],vec![6,5,4,3,2,1,7]));
    println!("Find words:{:?}", hash_table_problems::find_words(vec!["adsdf".to_string(),"sfd".to_string()]));
    println!("distribute candies:{:?}", hash_table_problems::distribute_candies(vec![6,6,6,6]));
    println!("Find harmonies:{:?}", hash_table_problems::find_lhs(vec![1,1,1,1]));
    //Math related problems
    println!("Number is palindrome:{:?}", math_related_problems::is_palindrome(121));
    println!("sqrt of number:{:?}", math_related_problems::my_sqrt(14));
    println!("climb stairs:{:?}", math_related_problems::climb_stairs(3));
    println!("is happy number:{:?}", math_related_problems::is_happy(19));
    println!("power of two:{:?}", math_related_problems::is_power_of_two(6));
    println!("add digits:{:?}", math_related_problems::add_digits(0));
    println!("is Ugly:{:?}", math_related_problems::is_ugly(6));
    println!("can win nim:{:?}", math_related_problems::can_win_nim(6));
    println!("power of three:{:?}", math_related_problems::is_power_of_three(-1));
    println!("power of four:{:?}", math_related_problems::is_power_of_four(1));
    println!("perfect square:{:?}", math_related_problems::is_perfect_square(16));
    println!("add strings:{:?}", math_related_problems::add_strings("11".to_string(),"123".to_string()));
    println!("arrange coins:{:?}", math_related_problems::arrange_coins(8));
    println!("construct rectangle:{:?}", math_related_problems::construct_rectangle(122122));
    println!("Perfect Number:{:?}", math_related_problems::check_perfect_number(28));
    //sorting related problems
    println!("Perfect Number:{:?}", sorting_related_problems::third_max(vec![3,2,1]));
    println!("Find the content children:{:?}", sorting_related_problems::find_content_children(vec![1,2],vec![1,2,3]));
    println!("Relative Ranks:{:?}", sorting_related_problems::find_relative_ranks(vec![5,4,3,2,1]));
    println!("array pair sum:{:?}", sorting_related_problems::array_pair_sum(vec![6,2,6,5,1,2]));
    println!("maximum product:{:?}", sorting_related_problems::maximum_product(vec![-1,-2,-3]));
    println!("Find error nums:{:?}", sorting_related_problems::find_error_nums(vec![1,1]));
    println!("Find Dominant Value:{:?}", sorting_related_problems::dominant_index(vec![1,2,3,4]));
    println!("Fair candy swap:{:?}", sorting_related_problems::fair_candy_swap(vec![2],vec![1,3]));
    println!("sort by pairty:{:?}", sorting_related_problems::sort_array_by_parity(vec![0]));
    println!("sort by pairty II:{:?}", sorting_related_problems::sort_array_by_parity_ii(vec![2,3]));
    println!("Sorted Squares:{:?}", sorting_related_problems::sorted_squares(vec![-4,-1,0,3,10]));
    //two pointers
    println!("Reverse String:{:?}", two_pointers::reverse_str("abcdefg".to_string(),3));
    println!("Reverse words:{:?}", two_pointers::reverse_words("Let's take LeetCode contest".to_string()));
    println!("Valid Palindrome:{:?}", two_pointers::valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string()));
    //Binary Search
    let bad_version = Solution{
        bad_version: 1
    };
    println!("First bad version:{:?}", Solution::first_bad_version(&bad_version,1));
    println!("First bad version:{:?}", binary_search::search(vec![2,5],5));
    println!("Next Greatest Letter:{:?}", binary_search::next_greatest_letter(vec!['c','f','j'],'c'));
    println!("Next matrix solution:{:?}", binary_search::k_weakest_rows(
        vec![vec![1,1,0,0,0],
             vec![1,1,1,1,0],
             vec![1,0,0,0,0],
             vec![1,1,0,0,0],
             vec![1,1,1,1,1]],3));
    println!("Check if it exsist:{:?}", binary_search::check_if_exist(vec![0,0,-2,2]));
    println!("Count negatives:{:?}", binary_search::count_negatives(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]]));
    println!("find the distance:{:?}", binary_search::find_the_distance_value(vec![1,4,2,3],vec![-4,-3,6,10,20,30],3));
    println!("Find the kth positive number:{:?}", binary_search::find_kth_positive(vec![1,2,3,4],2));
    println!("Find the special array:{:?}", binary_search::special_array(vec![3,6,7,7,0]));
    println!("Find the target indices:{:?}", binary_search::target_indices(vec![1,2,5,2,3],5));
    println!("Answer Queries:{:?}", binary_search::answer_queries(vec![2,3,4,5],vec![1]));
    //prefix sum
    println!("Pivot index:{:?}", prefix_sum::pivot_index(vec![1,7,3,6,5,6]));
    println!("Minimum start value:{:?}", prefix_sum::min_start_value(vec![1,-2,-3]));
    println!("Max Score:{:?}", prefix_sum::max_score("011101".to_string()));
    println!("Running sum:{:?}", prefix_sum::running_sum(vec![3,1,2,10,1]));
    println!("Print sum Odd length subarrays:{:?}", prefix_sum::sum_odd_length_subarrays(vec![10,11,12]));
    println!("Print largest altitude:{:?}", prefix_sum::largest_altitude(vec![-4,-3,-2,-1,4,3,2]));
    println!("Print Maximum population:{:?}", prefix_sum::maximum_population(vec![vec![1950,1961],vec![1960,1971],vec![1970,1981]]));
    println!("Print Is covered or not:{:?}", prefix_sum::is_covered(vec![vec![1,50]],1,50));
    println!("Print left and right difference:{:?}", prefix_sum::left_right_difference(vec![1]));
    println!("Print the return to boundary:{:?}", prefix_sum::return_to_boundary_count(vec![3,2,-3,-4]));
    println!("Print the minimum sum of subarray:{:?}", prefix_sum::minimum_sum_subarray(vec![3, -2, 1, 4],2,3));
    println!("Print the Minimum Subarray:{:?}", prefix_sum::subarray_sum(vec![3,1,1,2]));
    println!("Print the Partition count:{:?}", prefix_sum::count_partitions(vec![2,4,6,8]));
    println!("Print the min subarray lenght:{:?}", prefix_sum::min_sub_array_len(11,vec![1,2,3,4,5]));
    //dynamic programming
    println!("Counts the bits which is 1:{:?}", dynamic_programming::count_bits(5));
    println!("Counts the fibonacci series:{:?}", dynamic_programming::fib(4));
    println!("Print the min cost to climb stairs:{:?}", dynamic_programming::min_cost_climbing_stairs(vec![10,15,20]));
    println!("Print the alice wins or not: {:?}", dynamic_programming::divisor_game(6));
    println!("Print the value of tribonacci series: {:?}", dynamic_programming::tribonacci(25));
    println!("Print the maximum repeating: {:?}", dynamic_programming::max_repeating("ababc".to_string(),"ab".to_string()));
    println!("Print the longest subsequence: {:?}", dynamic_programming::get_longest_subsequence(vec!["a".to_string(),"b".to_string(),"c".to_string(),"d".to_string()],vec![1,0,1,1]));
    println!("Print the longest palindrome: {:?}", dynamic_programming::longest_palindrome("cbbd".to_string()));
    println!("Print the generated paranthesis: {:?}", dynamic_programming::generate_parenthesis(2));
    println!("Print the required jump: {:?}", dynamic_programming::jump(vec![2,0,2,0,1]));
    println!("Print the sum of max subarray: {:?}", dynamic_programming::max_sub_array(vec![5,4,-1,7,8]));
    println!("Print the can jump last: {:?}", dynamic_programming::can_jump(vec![2,3,1,1,4]));
    println!("Print the max unique paths: {:?}", dynamic_programming::unique_paths(36,7));
    println!("Print the max unique paths without obstacles: {:?}", dynamic_programming::unique_paths_with_obstacles(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]));
    println!("Print the minium change distance: {:?}", dynamic_programming::min_distance("intention".to_string(),"execution".to_string()));
    //greedy problems
    println!("Can we place new flowers in the flower bed: {:?}", greedy::can_place_flowers(vec![1,0,0,0,0,1],2));
    println!("Can we give change correctly: {:?}", greedy::lemonade_change(vec![5,5,5,5,20,20,5,5,5,5]));
    println!("Can we Change DI value: {:?}", greedy::di_string_match("DDI".to_string()));
    println!("Can we give largest perimeter: {:?}", greedy::largest_perimeter(vec![2,1,2]));
    println!("Can we change k nagations and sum: {:?}", greedy::largest_sum_after_k_negations(vec![2,-3,-1,5,-4],2));
    println!("Can we partition into 3 with equal sum: {:?}", greedy::can_three_parts_equal_sum(vec![3,3,6,5,-2,2,5,1,-9,4]));
    println!("Can we min cost to move chips: {:?}", greedy::min_cost_to_move_chips(vec![1,1000000000]));
    println!("How many balanced split we can have: {:?}", greedy::balanced_string_split("LLLLRRRR".to_string()));
    println!("Maximum number we got using 6 and 9: {:?}", greedy::maximum69_number(9999));
    //medium problems
    println!("Maximum area we got: {:?}", greedy::max_area(vec![1,8,6,2,5,4,8,3,7]));
    println!("Maximum profit we got: {:?}", greedy::max_profit(vec![7,6,4,3,1]));
    println!("Can complete circuit in gas station: {:?}", greedy::can_complete_circuit(vec![1,2,3,4,5],vec![3,4,5,1,2] ));

    //breadth first search
    println!("print the binary tree inorder traversal: {:?}", breadth_first_search::inorder_traversal(
        TreeNode::build_tree_from_array(&[Some(1),Some(2),Some(3),Some(4),Some(5),None,Some(8),None,None,Some(6),Some(7),Some(9)])));
    println!("print Is the given binary trees are same : {:?}", breadth_first_search::is_same_tree(
        TreeNode::build_tree_from_array(&[Some(2),None,Some(3),None,Some(4),None,Some(5),None,Some(6)]),
        TreeNode::build_tree_from_array(&[Some(2),None,Some(3),None,Some(4),None,Some(5),None,Some(6)])));
    println!("Is the binary tree symmetric or not: {:?}", breadth_first_search::is_symmetric(
        TreeNode::build_tree_from_array(&[Some(2),None,Some(3),None,Some(4),None,Some(5),None,Some(6)])));
    println!("print the binary tree maximum depth: {:?}", breadth_first_search::max_depth(
        TreeNode::build_tree_from_array(&[Some(2),None,Some(3),None,Some(4),None,Some(5),None,Some(6)])));
    println!("Is the binary tree balanced: {:?}", breadth_first_search::is_balanced(
        TreeNode::build_tree_from_array(&[Some(2),None,Some(3),None,Some(4),None,Some(5),None,Some(6)])));
    println!("print the binary tree minium depth: {:?}", breadth_first_search::min_depth(
        TreeNode::build_tree_from_array(&[Some(2),None,Some(3),None,Some(4),None,Some(5),None,Some(6)])));
    println!("print whether the binary tree has target sum: {:?}", breadth_first_search::has_path_sum(
        TreeNode::build_tree_from_array(&[Some(5),Some(4),Some(8),Some(11),None,Some(13),Some(4),Some(7),Some(2),None,None,None,Some(1)]),22));
    println!("print the binary tree pre-order traversal: {:?}", breadth_first_search::preorder_traversal(
        TreeNode::build_tree_from_array(&[Some(1),Some(2),Some(3),Some(4),Some(5),None,Some(8),None,None,Some(6),Some(7),Some(9)])));
    println!("print the binary tree pre-order traversal: {:?}", breadth_first_search::postorder_traversal(
        TreeNode::build_tree_from_array(&[Some(1),Some(2),Some(3),Some(4),Some(5),None,Some(8),None,None,Some(6),Some(7),Some(9)])));
    println!("print the inverted binary tree: {:?}", breadth_first_search::invert_tree(
        TreeNode::build_tree_from_array(&[Some(4),Some(2),Some(7),Some(1),Some(3),Some(6),Some(9)])));
    println!("print the path of binary tree: {:?}", breadth_first_search::binary_tree_paths(
        TreeNode::build_tree_from_array(&[Some(1),Some(2),Some(3),None,Some(5)])));
    println!("print the mode in the binary tree: {:?}", breadth_first_search::find_mode(
        TreeNode::build_tree_from_array(&[Some(0)])));
    println!("print the path of binary tree: {:?}", breadth_first_search::sum_of_left_leaves(
        TreeNode::build_tree_from_array(&[Some(1)])));
    println!("print the minimum difference of binary tree: {:?}", breadth_first_search::get_minimum_difference(
        TreeNode::build_tree_from_array(&[Some(236),Some(104),Some(701),None,Some(227),None,Some(911)])));
    //Medium
    println!("print is valid binary search tree: {:?}", breadth_first_search::is_valid_bst(
        TreeNode::build_tree_from_array(&[Some(2),Some(1),Some(3)])));
    let mut bst = TreeNode::build_tree_from_array(&[Some(1),Some(3),None,None,Some(2)]);
    breadth_first_search::recover_tree(&mut bst);
    println!("print is recovered binary search tree: {:?}", bst);
    println!("print is whether the word exsist in grid: {:?}", breadth_first_search::exist(
        vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCCED".to_string()));
    println!("print the path to find the target: {:?}", breadth_first_search::path_sum(
        TreeNode::build_tree_from_array(&[Some(5),Some(4),Some(8),Some(11),None,Some(13),Some(4),Some(7),Some(2),None,None,Some(5),Some(1)]),22));
    let mut linked_list = TreeNode::build_tree_from_array(&[Some(1),Some(2),Some(5),Some(3),Some(4),None,Some(6)]);
    breadth_first_search::flatten(&mut linked_list);
}
