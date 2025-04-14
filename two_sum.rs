use std::collections::HashMap;
fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    
    let mut map = HashMap::new(); 

    for i in 0..nums.len(){
        let cur_num = nums[i];
        let cur_val = target - cur_num;

        if let Some(val) = map.get(&cur_val){
            return vec![nums[*val] as i32,i as i32]
        }else{
            map.insert(cur_num,i);
        }

    }


    vec![]
}

pub fn main(){
    two_sum(vec![2,7,11,15],9);
}


