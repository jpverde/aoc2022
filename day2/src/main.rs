fn main() {
    let result = get_result();
    let result_two = get_result_two();

    println!("Result 1 {result:?}");
    println!("Result 2 {result_two:?}");
}

fn get_result() -> u64 {
    let lines = include_str!("input.txt")
        .lines()
        .map(|line| match line {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        })
        .sum::<u64>();

    lines
}

fn get_result_two() -> u64 {
    let lines = include_str!("input.txt")
        .lines()
        .map(|line| match line {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        })
        .sum::<u64>();

    lines
}
