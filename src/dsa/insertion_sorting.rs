pub fn insertion_sort(mut arr:Vec<i32>)->Vec<i32>{
    let n = arr.len();
    for i in 1..n as i32 {
        let mut insert_index = i;
        let current_value:i32 = arr[i as usize];
        let mut previous_element = i-1;

        while previous_element>=0 && arr[previous_element as usize] > current_value {
            arr[previous_element as usize +1] = arr[previous_element as usize];
            insert_index = previous_element;
            previous_element -=1;
        }
        arr[insert_index as usize] = current_value;
    } 
    arr
}