use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

    println!("{:?}",strs);

    if strs.len() ==1{
        return vec![strs];
    }
    let mut map:HashMap<String,Vec<String>> = HashMap::new();
    let mut vec:Vec<Vec<String>> = Vec::new();

    for str in strs{
        let mut s:Vec<char> = str.chars().collect();
        s.sort();
        let s1:String = s.into_iter().collect();
        map.entry(s1).or_default().push(str.to_string());
    }

    map.into_values().collect()
}

fn main(){
    //println!("{:?}",group_anagrams(vec!["a".to_string()]));
    println!("{:?}",group_anagrams(vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()]));
}