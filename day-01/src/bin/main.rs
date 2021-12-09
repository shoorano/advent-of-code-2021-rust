use itertools::Itertools;

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(a, b, c, d)| a + b + c < b + c + d)
        .count()
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
}
