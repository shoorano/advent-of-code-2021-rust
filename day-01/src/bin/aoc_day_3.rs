const WIDTH: usize = 12;

fn part_1(input: &str) -> i32 {
    let mut elements = vec![0; WIDTH];
    let _mappings: () = input
        .lines()
        .map(|s| -> Vec<char> {s.chars().collect()})
        .map(|nums| {
            for (i, num) in nums.iter().enumerate() {
                elements[i] = elements[i] + num.to_digit(10).unwrap() as i32;
            }
        }
    )
    .collect();

    let base = 2 as i32;
    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;
    let count = input.lines().count() as i32;
    let _gamma_rate: () = elements
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let power = (WIDTH - 1 - i) as u32;
            if v < &(&count / 2) {
                epsilon_rate = epsilon_rate + base.pow(power);
            } else {
                gamma_rate = gamma_rate + base.pow(power);
            }
        })
        .collect();
    return gamma_rate * epsilon_rate;
}

fn part_2(input: &str) -> i32 {
    return get_o2(&input) * get_co2(&input);
}

fn get_o2(input: &str) -> i32 {
    let mut o2_data: Vec<&str> = input.lines().collect();
    for i in 0..WIDTH as usize {
        let mut zero_count = 0;
        let mut one_count = 0;
        if o2_data.len() == 1 {
            break;
        }
        let mut ones = vec![];
        let mut zeros = vec![];
        for line in o2_data.clone() {
            if line.chars().nth(i).unwrap() == '0' {
                zeros.push(line);
                zero_count = zero_count + 1;
            } else if line.chars().nth(i).unwrap() == '1' {
                ones.push(line);
                one_count = one_count + 1;
            }
        }
        if zero_count > one_count {
            o2_data = zeros;
        } else if one_count > zero_count {
            o2_data = ones;
        } else {
            o2_data = ones;
        }
    }
    return isize::from_str_radix(o2_data[0], 2).unwrap() as i32;
}

fn get_co2(input: &str) -> i32 {
    let mut co2_data: Vec<&str> = input.lines().collect();
    for i in 0..WIDTH as usize {
        let mut zero_count = 0;
        let mut one_count = 0;
        if co2_data.len() == 1 {
            break;
        }
        let mut ones = vec![];
        let mut zeros = vec![];
        for line in co2_data.clone() {
            if line.chars().nth(i).unwrap() == '0' {
                zeros.push(line);
                zero_count = zero_count + 1;
            } else if line.chars().nth(i).unwrap() == '1' {
                ones.push(line);
                one_count = one_count + 1;
            }
        }
        if zero_count > one_count {
            co2_data = ones;
        } else if one_count > zero_count {
            co2_data = zeros;
        } else {
            co2_data = zeros;
        }
    }
    return isize::from_str_radix(co2_data[0], 2).unwrap() as i32;
}

fn main() {
    let input = include_str!("../../data/input_day_3.txt");
    let result_1 = part_1(&input);
    println!("part 1 result: {:?}", result_1);
    let result_2 = part_2(&input);
    println!("part 2 result: {:?}", result_2);
}
