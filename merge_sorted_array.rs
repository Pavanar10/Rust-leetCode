pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

 //   let len = nums1.len()-m as usize;
   // let mut j = 0;

    for (j,i) in (m..m+n).enumerate(){
        println!("i {} j{}",i,j);
        nums1[i as usize] = nums2[j];
    }
    nums1.sort();
    println!("nums1 {:?} {}",nums1,m);
    



}

pub fn main(){
    merge(&mut vec![1,2,3,0,0,0],3,&mut vec![2,5,6],3);
}