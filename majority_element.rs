use std::collections::HashMap;
pub fn majority_element(nums: Vec<i32>) -> i32 {
   let mut map = HashMap::new();
    let  n = nums.len()/2;

    let mut max_count = 0;
   /* for num in &nums{
        let cnt = &nums.iter().filter(|&x| x ==num).count();
        if *cnt > n
            {
                max_count = *cnt 
            }
        }*/
    for num in nums{
       let  count =  map.entry(num).or_insert(0);
        *count += 1;
        if *count > n/2{
            max_count = num;
        }

    }
   // println!("{:?}",map);

    max_count as i32

    }

fn main(){
println!("{}",majority_element(vec![2,2,1,1,1,2,2]));

}