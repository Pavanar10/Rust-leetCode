pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    /*let mut i = 2;
    for j in 0..nums.len()-1{
        if nums[j] == nums[i]{
            nums[j] = nums[i+1];
            i+=1;
        }
    }


    println!("{:?},{}",nums,i);
    i as i32*/


    let mut j =1;

    for i in 1..nums.len(){

        if j ==1 || nums[i] != nums[j-2]{
            nums[j] = nums[i];
            j+=1;
        }
    }
    println!("{:?},{}",nums,j);
    j as i32
}

fn main(){
    remove_duplicates(&mut vec![1,1,1,2,2,3]);
}