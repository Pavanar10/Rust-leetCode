pub fn longest_common_prefix(strs: Vec<String>) -> String {
        
        let mut prev=strs[0].clone();
        for str in &strs[1..]{
            println!("here{:?}",prev);
            while !str.starts_with(&prev) {

                prev.pop();

                if prev.is_empty(){
                    return "".to_string()

                }
            }

        }
     prev
}

fn main(){
    println!("{:?}",longest_common_prefix(vec![String::from("flower"),String::from("flow"),String::from("flight")]));
}