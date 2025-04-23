pub fn xo(val: &'static str) -> bool {
    let val = val.to_lowercase();
    let x_val:i32 = val.chars().into_iter().fold(0,|mut acc,x| 
        {
            if x =='x'{
                acc+=1;
                acc
            }else{
                acc
            }
        });
    let y_val:i32 =val.chars().into_iter().fold(0,|mut acc,x| 
        {
            if x =='o'{
                acc+=1;
                acc
            }else{
                acc
            }
        }); 
    if x_val==y_val{
        return true;
    }else{
        return false;
    }
  }