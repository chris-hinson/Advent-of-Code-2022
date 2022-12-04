use std::collections::HashSet;
use std::fs;
fn main() {
    let sum: u32 = fs::read_to_string("./inputs/input3.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let cs: Vec<char> = l.chars().collect();
            let a: HashSet<&char> = HashSet::from_iter(cs[0..cs.len() / 2].iter());
            let b: HashSet<&char> = HashSet::from_iter(cs[cs.len() / 2..].iter());
            let intersect = a.intersection(&b).map(|e| **e).collect::<Vec<char>>()[0];
            if (65..=90).contains(&(intersect as u32)) {
                ((intersect as u32) - 65) + 27
            } else {
                (intersect as u32) - 96
            }
        })
        .sum();

    let sum2: u32 = fs::read_to_string("./inputs/input3.txt")
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()[..]
        .chunks(3)
        .into_iter()
        .map(|l| {
            let a: HashSet<char> = HashSet::from_iter(l[0].chars());
            let b: HashSet<char> = HashSet::from_iter(l[1].chars());
            let c: HashSet<char> = HashSet::from_iter(l[2].chars());
            let intersect1 = a.intersection(&b).map(|e| *e).collect::<HashSet<char>>();
            let intersect2 = intersect1
                .intersection(&c)
                .map(|e| *e)
                .collect::<Vec<char>>()[0];
            if (65..=90).contains(&(intersect2 as u32)) {
                ((intersect2 as u32) - 65) + 27
            } else {
                (intersect2 as u32) - 96
            }
        })
        .sum();

    println!("{sum}");
    println!("{sum2}");
}
