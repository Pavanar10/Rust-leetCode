
/* Given an input string s, reverse the order of the words.

A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

Return a string of the words in reverse order concatenated by a single space. */
pub fn reverse_words(s: String) -> String {
    let mut vec:Vec<String> = s.split_whitespace().map(String::from).collect();
    vec.reverse();
    return vec.join(" ")

}

fn main(){
    println!("{:?}",reverse_words(String::from("the sky is blue")));
}