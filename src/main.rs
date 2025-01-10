mod max_profit;

fn main() {
    let prices = vec![7,1,5,3,6,4];
    println!("Maximum profit: {}", max_profit::max_profit(prices));
}