//return median for a stream of inputs
use std::io::{self,BufRead};

fn main(){
    let input = io::stdin();
    for line in input.lock().lines(){
    let mut median:f64 = 0.0;
    
        match line{
            Ok(text)=>{
                println!("{:?}",text);
                let mut numbers:Vec<f64>=text.split(',').map(|s| s.trim().parse::<f64>().unwrap_or(0.0)).collect();
             
                println!("{:?}",numbers);
              //sort
                numbers.sort_by(|a,b| a.partial_cmp(b).unwrap());
              
              let mid = numbers.len()/2;
              let len = numbers.len();
              if len %2  == 1{
                  median = numbers[mid];
              }else{
                  let left = numbers[mid - 1];
                  let right = numbers[mid];
                  median = (left + right)/2.0;
                 }
              println!("{:?}",median);
            },
            Err(e)=>eprintln!("Error {}",e)
        }
    }

}