pub fn length_of_last_word(s: String) -> i32 {

  /*  let vec:Vec<String> = s.split_whitespace().map(String::from).collect();
    println!("{:?}",vec);

    vec[vec.len()-1].len() as i32 */

    s.split_whitespace().next_back().unwrap().len() as i32
}

fn main(){
    println!("{}",length_of_last_word(String::from("Hello word")));
}