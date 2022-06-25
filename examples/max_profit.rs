// LeetCode problem https://leetcode.com/problems/best-time-to-buy-and-sell-stock/ using Monoids
use std::env;

use partial_functional::prelude::*;

fn max_profit(prices: Vec<u32>) -> u32 {
    let mut cheapest = Min::empty();
    let mut profit = Max::empty();

    for price in prices {
        cheapest = cheapest.combine(price.into());
        profit = profit.combine(Max(price - cheapest.0));
    }

    profit.0
}

fn main() {
    let args = env::args().skip(1).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    if args.is_empty() {
        println!("No arguments given");
        return;
    }

    println!("Max profit: {}", max_profit(args));
}
