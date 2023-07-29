use std::cmp::Reverse;

use itertools::Itertools;

fn main() {
    let result = get_result();

    println!("{result:?}")
}

fn get_result() -> u64 {
    let result = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();

    result
}
#[cfg(test)]
mod tests {
    use crate::get_result;

    #[test]
    fn test_result() {
        let result = get_result();

        assert_eq!(result, 206104)
    }
}
