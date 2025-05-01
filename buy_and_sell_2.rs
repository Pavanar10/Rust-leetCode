pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit=0;
    let mut right = 1;

    for left in 0..prices.len()-1{
        let p = prices[right]-prices[left];
        if p>0{
            max_profit += p;
        }
        right +=1;
    }
    max_profit

}

fn main(){

    println!("{}",max_profit(vec![7,1,5,3,6,4]));
}