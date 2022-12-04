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

    let score2: i32 = fs::read_to_string("./inputs/input2.txt")
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
            //X lose, Y tie, Z win
            ('A', 'X') => ('A', 'S'),
            ('A', 'Y') => ('A', 'R'),
            ('A', 'Z') => ('A', 'P'),
            ('B', 'X') => ('B', 'R'),
            ('B', 'Y') => ('B', 'P'),
            ('B', 'Z') => ('B', 'S'),
            ('C', 'X') => ('C', 'P'),
            ('C', 'Y') => ('C', 'S'),
            ('C', 'Z') => ('C', 'R'),
            _ => panic!("how the fuck did you manage this"),
        })
        .map(|game| match game {
            ('A', 'R') => 4,
            ('A', 'P') => 8,
            ('A', 'S') => 3,
            ('B', 'R') => 1,
            ('B', 'P') => 5,
            ('B', 'S') => 9,
            ('C', 'R') => 7,
            ('C', 'P') => 2,
            ('C', 'S') => 6,
            _ => panic!("how the fuck did you manage this"),
        })
        .sum();

    println!("{:?}", score);
    println!("{score2}");
}
