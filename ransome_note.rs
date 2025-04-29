use std::collections::HashMap;
pub fn can_construct(ransom_note: String, magazine: String) -> bool {

    let mut map = HashMap::new();

    for c in magazine.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    println!("{:?}",map);
    let mut res = String::new();
    for c in ransom_note.chars(){

        match map.get_mut(&c){
            Some(val)  if *val > 0 =>  { *val -= 1},
            _=>{return false }
        }
    }
    true




}

fn main(){
    println!("{}",can_construct(String::from("aa"),String::from("aab")));
}