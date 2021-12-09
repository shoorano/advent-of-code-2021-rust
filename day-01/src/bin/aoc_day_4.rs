use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/input_day_4.txt");
    let data: Vec<&str> = input.lines().collect();
    let numbers = data[0];
    let mut cards = vec![];
    for card in data[1..].chunks(6).into_iter() {
        cards.push(&card[1..]);
    }
    println!("{:?}", numbers);
    for card in cards.iter() {
        println!("card: {:?} \n", card);
    }
}