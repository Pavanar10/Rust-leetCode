pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
/*    let mut j = 0;
    for i in 0..nums.len(){
        if nums[i] != val{
            nums[j] = nums[i];
            j+=1;
        }
    }
    println!("{:?}",nums);
    j as i32*/

    //or
    nums.retain(|&x| x  != val);
    nums.len() as i32
}

fn main(){
println!("{}",remove_element(&mut vec![3,2,2,3],3));
}