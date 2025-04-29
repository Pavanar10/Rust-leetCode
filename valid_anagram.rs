pub fn is_anagram(s: String, t: String) -> bool {

    let mut map = std::collections::HashMap::new();

    if s.len() != t.len(){
        return false
    }

    for c in s.chars(){
        *map.entry(c).or_insert(0) += 1;
    }
    println!("{:?}",&map);

    for c in t.chars(){
        match map.get_mut(&c){
            Some(n) if *n>0 =>{ *n -= 1},
            _=>{return false}
        }
    }

    true

}

fn main(){
    println!("{:?}",is_anagram(String::from("anagram"),String::from("naggrat")));
}