use std::collections::HashSet;
fn find_disappeared_numbers(nums:  Vec<i32>) -> Vec<i32> {
    let len = nums.len() ;
    let mut set:HashSet<i32>= nums.into_iter().collect();
    let mut vec = Vec::new();
    println!("{:?}",set);
    /*(1..=len)
    .filter(|&num| !set.contains(&(num as i32)))
    .map(|x| x as i32) // already i32, but for clarity
    .collect()*/

    for i in 1..=len{
        if !set.contains(&(i as i32)){
            vec.push(i as i32)
        }
    }


    vec

}

fn main(){
    println!("{:?}",find_disappeared_numbers(vec![4,3,2,1,2,7,8]));
}