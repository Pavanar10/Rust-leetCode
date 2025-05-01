pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit =0;
    let mut left =0;
    let mut right = 1;

    for _i in 0..prices.len()-1{
        if prices[left]<prices[right]{
            let profit  = prices[right] -prices[left];
            max_profit = std::cmp::max(profit,max_profit);
        }
        else{
            left = right;
        }
        right+=1;
    }
max_profit
}

fn main(){
    println!("{}",max_profit(vec![7,1,5,3,6,4]))
}