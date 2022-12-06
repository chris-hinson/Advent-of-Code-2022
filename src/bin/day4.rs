use regex::Regex;
use std::{fs, ops::RangeInclusive};
pub fn main() {
    /*let pairs: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> =
    fs::read_to_string("./inputs/input4.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let group = l
                .split(",")
                .map(|elf| {
                    let elves = elf
                        .split("-")
                        .map(|r| r.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    (elves[0], elves[1])
                })
                .collect::<Vec<(usize, usize)>>();
            (group[0].0..=group[0].1, group[1].0..=group[1].1)
        })
        .collect();*/
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    let pairs: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> =
        fs::read_to_string("./inputs/input4.txt")
            .unwrap()
            .lines()
            .map(|l| {
                let caps = re.captures(l).unwrap();
                (
                    (caps.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        ..=caps.get(2).unwrap().as_str().parse::<usize>().unwrap()),
                    (caps.get(3).unwrap().as_str().parse::<usize>().unwrap()
                        ..=caps.get(4).unwrap().as_str().parse::<usize>().unwrap()),
                )
            })
            .collect();

    let total_overlaps: usize = pairs
        .iter()
        .filter(|(a, b)| {
            a.clone().all(|e| b.clone().contains(&e)) || b.clone().all(|e| a.clone().contains(&e))
        })
        .count();

    let partial_overlaps: usize = pairs
        .iter()
        .filter(|(a, b)| a.clone().any(|e| b.contains(&e)) || b.clone().any(|e| a.contains(&e)))
        .count();

    //503
    println!("total overlaps: {total_overlaps}");
    //827
    println!("partial overlaps: {partial_overlaps}")
}
