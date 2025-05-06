var generate = function(numRows) {
    let number = [];

    for(let i =0;i<numRows;i++){
        let res = [];
        res[0]=1;
        res[i]=1;

        for(let j= 1;j<i;j++){
            res[j]=number[i-1][j-1]+number[i-1][j];
        }
        number.push(res);

    }
    return number

}

fn main(){
    println!("{:?}",function(5));
}