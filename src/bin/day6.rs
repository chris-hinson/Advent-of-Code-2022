#![feature(slice_partition_dedup)]
use std::fs;
pub fn main() {
    let pos = *&mut fs::read_to_string("./inputs/input6.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|w| {
            let test = &mut w.clone().to_vec()[..];
            test.sort();
            !(test.partition_dedup().1.len() > 0)
        })
        .unwrap()
        + 4;

    let pos2 = *&mut fs::read_to_string("./inputs/input6.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|w| {
            let test = &mut w.clone().to_vec()[..];
            test.sort();
            !(test.partition_dedup().1.len() > 0)
        })
        .unwrap()
        + 14;

    println!("pos: {pos}");
    println!("pos2: {pos2}")
}
