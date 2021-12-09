use std::collections::HashMap;

fn part_1(input: &str) -> i32 {
    let mut map = HashMap::new();
    map.insert("up", 0);
    map.insert("forward", 0);
    map.insert("down", 0);
    let _mappings: () = input
        .lines()
        .map(|s| -> Vec<&str> {s.split(" ").collect()})
        .map(|v| {
            let num = v[1].parse::<i32>().unwrap();
            map.insert(v[0], map[v[0]] + num);
        })
        .collect();
    return (map["down"] - map["up"])*map["forward"];
}

fn part_2(input: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;
    let _mappings: () = input
        .lines()
        .map(|s| -> Vec<&str> {s.split(" ").collect()})
        .map(|v| {
            let direction: &str = v[0];
            let num = v[1].parse::<i32>().unwrap();
            match direction {
                "up" => aim = aim - num,
                "down" => aim = aim + num,
                "forward" => {
                    horizontal = horizontal + num;
                    depth = depth + num * aim;
                }
                _ => ()
            }
        })
        .collect();
    return horizontal * depth;
}


fn main() {
    let input = include_str!("../../data/input_day_2.txt");
    let result_1 = part_1(&input);
    println!("{:?}", result_1);
    let result_2 = part_2(&input);
    println!("{:?}", result_2);
}

