

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
   /*  let mut  i = 0;
    for j in 0..nums.len()-1{
        if nums[j] ==nums[i]{
            nums[i] = nums[i+1];
            i+=1;
        }
    }
    println!("{:?}",nums);
    i as i32*/

    let mut i =0;

    match nums.is_empty(){
        true =>0,
        false =>{
            for j in 1..nums.len(){
                if nums[i] != nums[j]{
                    i+=1;
                    nums[i] = nums[j];
                }
            }
            (i+1) as i32
        }
    }
}


fn main(){
    println!("{}",remove_duplicates(&mut vec![1,1,2]));
}