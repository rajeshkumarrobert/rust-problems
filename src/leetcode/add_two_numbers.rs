// Definition for singly-linked list.
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

pub struct Solution{

}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut val1 = vec![l1.clone().unwrap().val];
        let mut value = l1.clone();
        while value.clone().unwrap().next.is_some(){
           val1.push(value.clone().unwrap().next.unwrap().val);
           println!("Test l1 iterator {:?}",value.clone().unwrap().next);
           value = value.clone().unwrap().next;
        }
        let mut val2 = vec![l2.clone().unwrap().val];
        let mut value2 = l2.clone();
        while value2.clone().unwrap().next.is_some(){
            println!("Test l2 iterator {:?}",value2.clone().unwrap().next);
            val2.push(value2.clone().unwrap().next.unwrap().val); 
            value2 = value2.clone().unwrap().next;
         }
         let num1:i64 = val1.iter().map(|x|x.to_string()).collect::<String>().parse().unwrap();
         let num2:i64 = val2.iter().map(|x|x.to_string()).collect::<String>().parse().unwrap();
         println!("the l1 is {num1},the l2 is{num2}");
         let  result = num1+num2;
         let result_char:Vec<char> =result.to_string().chars().rev().collect();
         println!("{:?}",result_char);
         let length = result_char.len();
         let mut final_res = ListNode{
            val:0,
            next:None,

         };
         let mut result_listnode : Option<Box<ListNode>> = None;     
    //      for i in  length..0{
    //         let current_val:i32 = result_char.get(i).unwrap().to_digit(10).unwrap() as i32;
    //          //final_res = ListNode::new(val);
    //          final_res = ListNode{
    //             val:current_val,
    //             next:{
    //                 if result_char.get(i+1).is_some() {
    //                     println!("test {}",result_char.get(i+1).unwrap().to_digit(10).unwrap() as i32);
    //                     Some(Box::new(ListNode::new(result_char.get(i+1).unwrap().to_digit(10).unwrap() as i32)))
    //                 }else{
    //                     None
    //                 }
    //          },
    //      };
    //    //result_listnode = result_listnode.insert(*final_res);
    // }
     let mut current_index=length-1;
    loop {
      let current_val:i32 = result_char.get(current_index).unwrap().to_digit(10).unwrap() as i32;
      println!("The current val is {current_val}");
      final_res = ListNode{
                   val:current_val,
                    next:{
                        if current_index!=length-1 {
                            println!("test {}",result_char.get(current_index+1).unwrap().to_digit(10).unwrap() as i32);
                            Some(Box::new(ListNode::new(result_char.get(current_index+1).unwrap().to_digit(10).unwrap() as i32)))
                        }else{
                            None
                        }
                      }
                    };
      if current_index==0{
        break;
      }
      current_index=current_index-1;
  }
    Some(Box::new(final_res))
}
}