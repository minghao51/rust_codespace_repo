use itertools::Itertools;
fn main() -> color_eyre::Result<()> {
    // https://fasterthanli.me/series/advent-of-code-2022/part-1
    // https://www.ericburden.work/blog/2022/12/01/advent-of-code-2022-day-01/

    color_eyre::install()?;

    let max = std::fs::read_to_string("data.txt").unwrap() //# include_str!("")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max();
    println!("{max:?}");

    Ok(())
}