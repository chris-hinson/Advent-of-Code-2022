use std::fs;

use regex::Regex;
pub fn main() {
    //read from file and split up into the stacks and moves
    let contents = fs::read_to_string("./inputs/input5.txt").unwrap();
    let split = contents.split_once("\n\n").unwrap();
    let stacks_in = split.0;
    let moves = split.1;

    ////////////////////////////////stack input processing///////////////////////////////
    //reverse the lines so we can pop off them to build up
    let stacks_in: Vec<&str> = stacks_in.lines().rev().collect();
    let num_stacks = stacks_in[0].split("   ").count();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    for line in &stacks_in[1..] {
        let this_line: Vec<char> = line.chars().collect::<Vec<char>>();
        let this_line: Vec<&[char]> = this_line[..].chunks(4).collect();
        let this_line: Vec<char> = this_line.iter().map(|e| e[1]).collect();

        for (i, c) in this_line.iter().enumerate() {
            if c.is_alphabetic() {
                stacks[i].push(*c);
            }
        }
    }
    let mut stacks2 = stacks.clone();

    //////////////////////////////move input processing//////////////////////////////////
    let move_re = Regex::new(r"\w* (\d*) \w* (\d*) \w* (\d*)").unwrap();
    let moves: Vec<(usize, usize, usize)> = moves
        .lines()
        .map(|l| {
            let caps = move_re.captures(l).unwrap();
            (
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            )
        })
        .collect();
    //////////////////////////////make the actual moves//////////////////////////////////
    for (num, from, to) in moves.iter() {
        for _i in 0..*num {
            let temp = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(temp);
        }
    }

    //get our  answer
    println!("part 1: ");
    for stack in stacks.iter_mut() {
        print!("{}", stack.pop().unwrap());
    }
    println!();
    //////////////////////////////make the actual moves part 2///////////////////////////
    for (num, from, to) in moves.iter() {
        let mut move_stack = Vec::new();
        for _i in 0..*num {
            let temp = stacks2[from - 1].pop().unwrap();
            move_stack.push(temp);
        }
        move_stack.reverse();
        for e in move_stack {
            stacks2[to - 1].push(e);
        }
    }
    //get our actual answer
    println!("part 2: ");
    for stack in stacks2.iter_mut() {
        print!("{}", stack.pop().unwrap());
    }
}
