pub fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    let mut positive_count=0;
    let mut negative_sum=0;
    for i in &input{
        if i.is_positive(){
            positive_count+=1;
        }
        if i.is_negative(){
           negative_sum = negative_sum + i;
        }
    }
    if input==[]{
       result=[].to_vec();
    }else {
        result.push(positive_count);
        result.push(negative_sum);
    }
    println!("{:?}",result);
    result
}