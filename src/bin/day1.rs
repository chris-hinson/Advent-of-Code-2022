use std::fs;

fn main() {
    let mut elves = fs::read_to_string("./inputs/input1.txt").unwrap().split("\n\n").map(|e| e.lines().collect::<Vec<&str>>()).map(|e| {e.iter().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>()}).map(|e| e.iter().sum()).collect::<Vec<i32>>();

    /*let contents = contents.split("\n\n").collect::<Vec<&str>>();
    let elves = contents
        .iter()
        .map(|e| e.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let elves = elves
        .iter()
        .map(|e| {
            e.iter()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut elves = elves.iter().map(|e| e.iter().sum()).collect::<Vec<i32>>();*/
    elves.sort();
    //elves.reverse();

    //println!("{:?}", elves);

    println!("max calories: {}", elves.iter().max().unwrap());
    println!(
        "top 3: {}",
        elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
    );
}
