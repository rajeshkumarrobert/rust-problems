use codewars::{count_positive_sum_negative::count_positives_sum_negatives, make_negative::make_negative, maps, paperwork::paperwork, reverse_array::digitize
};
mod codewars;
fn main() {
    //println!("Hello, world!");
    digitize(348597);
    count_positives_sum_negatives(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15]);
    println!("{}",paperwork(5,-5 ));
    println!("{:?}",maps::maps(&vec![1, 2, 3, 4]));
    println!("{}",make_negative(1));
}
