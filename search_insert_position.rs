use std::cmp::Ordering;
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {

   /* let res = match nums.binary_search(&target){
        Ok(index)=>index,
        Err(index)=>index,
    };

    res as i32*/
    let  (mut left,mut right)=(0,nums.len());

    while left < right{
        println!("left right {} {}",left,right);

        let mid = (left+right)/2;
        println!("{}",mid);
       /* match nums[mid].cmp(&target){
            Ordering::Less => left=mid+1,
            _=>right = mid,

        };*/
        if nums[mid] == target{
            left = mid;

        }else if nums[mid] < target{
            left = mid +1;
            
            }
            else {        right= mid; }
    }
    left as i32
}

fn main(){
    println!("{}",search_insert(vec![1,3,5,6],7));
}