pub fn bubble_sort(mut array:Vec<usize>)->Vec<usize>{
    let n = array.len();
    
    // Return early if array is empty or has single element
    if n <= 1 {
        return array;
    }

    // Outer loop for passes
    for i in 0..n-1 {
        // Flag to optimize when array is already sorted
        let mut swapped = false;
        
        // Inner loop for comparisons
        for j in 0..n-i-1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // If no swapping occurred, array is sorted
        if !swapped {
            break;
        }
    }
    
    array
}