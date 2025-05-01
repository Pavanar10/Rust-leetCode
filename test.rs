pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

    let mut j =0;

    for i  in (m-n)..m{
        println!("{}",nums1[i as usize]);
        nums1[i as usize] = nums2[j];
        j+=1;
    }
nums1.sort();
println!("{:?}",nums1);

}

fn main(){
    merge(&mut vec![1,2,3,0,0,0],6,&mut vec![2,5,6],3);
}