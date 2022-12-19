use std::fs;
use std::collections::BinaryHeap;

fn main() {
    let mut elves = fs::read_to_string("./inputs/input1.txt").unwrap().split("\n\n").map(|e| e.lines().collect::<Vec<&str>>()).map(|e| {e.iter().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>()}).map(|e| e.iter().sum()).collect::<BinaryHeap<i32>>().into_sorted_vec();

    println!("max calories: {}", elves.iter().max().unwrap());
    println!(
        "top 3: {}",
        elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
    );
}
