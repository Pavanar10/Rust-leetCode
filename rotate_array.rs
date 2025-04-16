

pub fn rotate(nums: &mut Vec<i32>, k: i32) {

    let len = nums.len();
    let  k = (k as usize) % len;
    if len ==1{
        return;
    }
    if k == 0{
        return;
    }
    nums.reverse();

    println!("{:?}",nums);
    reverse(nums,0,k-1);
    println!("{:?}",nums);
    reverse(nums,k,len-1);
    println!("{:?}",nums);

}

pub fn reverse(nums:&mut Vec<i32>,mut start:usize,mut end:usize){
    while start < end{
        nums.swap(start,end);
        start +=1;
        end-=1;
    }

}

fn main(){
    rotate(&mut vec![1,2,3,4,5,6,7],3);
}