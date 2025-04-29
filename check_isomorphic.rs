pub fn is_isomorphic(s: String, t: String) -> bool {

    if s.len() != t.len(){
        return false
    }

    let mut map1= std::collections::HashMap::new();
    let mut map2= std::collections::HashMap::new();

    //check indexs

    for ((i,c),d) in s.chars().enumerate()
    .zip(t.chars()){
        let index1 = map1.entry(c).or_insert(Vec::new());
        index1.push(i);
        println!("{:?} vec1",index1);

        let index2 = map2.entry(d).or_insert(Vec::new());
        index2.push(i);
        println!("{:?} vec2",index2);

        if index1 != index2{
            return false
        }
    }

    true
}

fn main(){

    println!("{:?}",is_isomorphic(String::from("egg"),String::from("add")));
}