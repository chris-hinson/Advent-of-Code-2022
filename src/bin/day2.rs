use std::fs;

fn main() {
    let score: i32 = fs::read_to_string("./inputs/input2.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let chars = l
                .split(" ")
                .map(|c| {
                    let char = c.parse::<char>().unwrap();
                    char
                })
                .collect::<Vec<char>>();
            (chars[0], chars[1])
        })
        .map(|game| match game {
            ('A', 'X') => 4,
            ('A', 'Y') => 8,
            ('A', 'Z') => 3,
            ('B', 'X') => 1,
            ('B', 'Y') => 5,
            ('B', 'Z') => 9,
            ('C', 'X') => 7,
            ('C', 'Y') => 2,
            ('C', 'Z') => 6,
            _ => panic!("how the fuck did you manage this"),
        })
        .sum();

    println!("{:?}", score);
}
