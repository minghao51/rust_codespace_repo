use itertools::Itertools;
use std::cmp::Reverse;

pub fn solve2() -> color_eyre::Result<()> {
    let answer = std::fs::read_to_string("data.txt").unwrap() //# include_str!("")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
    println!("{answer:?}");    
    Ok(())
}