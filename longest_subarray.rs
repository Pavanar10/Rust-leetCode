use std::collections::HashSet;
pub fn length_of_longest_substring(s: String) -> i32 {

    let mut set:HashSet<char> = HashSet::new();
    let mut left =0;
    let mut max_length = 0;

    for (right,char) in s.chars().enumerate(){
        println!("{}",char);
        println!("set {:?}",set);

        while set.contains(&char){
            set.remove(&s.chars().nth(left).unwrap());
            left+=1;
        }
        set.insert(char);
        max_length = std::cmp::max(max_length, right-left+1);
    }



    max_length as i32
}

fn main(){
    println!("Length {}",length_of_longest_substring(String::from("abcabcbb")));
}