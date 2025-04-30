pub fn is_subsequence(s: String, t: String) -> bool {


    let mut i=0;
    let mut j =0;

   /* for char in t.chars(){
        if let Some(ch)=s.chars().nth(j){
            if char == ch{
                j+=1;
                
            }
            if j == s.len(){
                return true;
            }

            i += 1;
        }
    }*/

    let s1 = s.as_bytes();
    let t1 = t.as_bytes();
    for &b in t1{
        if s1[j] == b {
            j+=1;
        }
        if j==s.len(){
            return true;
        }
        
    }
    false

}

fn main(){
    println!("{:?}",is_subsequence(String::from("abc"),String::from("ahbgdc")));
}