use colored::*;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

pub fn main() {
    let input = include_str!("../../inputs/input9.txt");

    let commands: Vec<char> = input
        .lines()
        .map(|e| {
            let parts = e.split(" ").collect::<Vec<&str>>();
            vec![parts[0].as_bytes()[0] as char; parts[1].parse::<usize>().unwrap()]
        })
        .flatten()
        .collect::<Vec<char>>();
    println!("{:?}", commands);

    let mut positions: Vec<(i32, i32)> = vec![(0, 0); 10];

    //let mut head_pos: (i32, i32) = (0, 0);
    //let mut tail_pos: (i32, i32) = (0, 0);
    let mut x_bound: (i32, i32) = (0, 0);
    let mut y_bound: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for char in commands {
        //move the haed
        match char {
            'U' => {
                positions[0].1 += 1;
                y_bound.1 = max(y_bound.1, positions[0].1);
            }
            'D' => {
                positions[0].1 -= 1;
                y_bound.0 = min(y_bound.0, positions[0].1);
            }
            'L' => {
                positions[0].0 -= 1;

                x_bound.0 = min(x_bound.0, positions[0].0);
            }
            'R' => {
                positions[0].0 += 1;

                x_bound.1 = max(x_bound.1, positions[0].0);
            }
            _ => panic!("bad command"),
        }

        //println!("head move: ");
        //print_board(head_pos, tail_pos);

        //move the next knot if nessecary
        //check if we are in the same row
        for knot_num in 1..10 {
            if positions[knot_num - 1].1 == positions[knot_num].1 {
                //move either left or right
                positions[knot_num].0 +=
                    if (positions[knot_num - 1].0 - positions[knot_num].0).abs() > 1 {
                        if positions[knot_num - 1].0 < positions[knot_num].0 {
                            -1
                        } else {
                            1
                        }
                    } else {
                        0
                    };
            }
            //check if we are in the same column
            else if positions[knot_num - 1].0 == positions[knot_num].0 {
                //move either up or down
                positions[knot_num].1 +=
                    if (positions[knot_num - 1].1 - positions[knot_num].1).abs() > 1 {
                        if positions[knot_num - 1].1 < positions[knot_num].1 {
                            -1
                        } else {
                            1
                        }
                    } else {
                        0
                    };
            }
            //if we are in neither, do something??
            else {
                //CHEBYSHEV DISTANCE LETS GO
                let dist = max(
                    (positions[knot_num - 1].0 - positions[knot_num].0).abs(),
                    (positions[knot_num - 1].1 - positions[knot_num].1).abs(),
                );
                //println!("dist: {dist}");
                //let dist = (((tail_pos.0 - head_pos.0).pow(2) + (tail_pos.1 - head_pos.1).pow(2))as f32).sqrt();
                //if euclidean distance is > 1
                if dist > 1 {
                    let slope: f32 = (positions[knot_num - 1].1 - positions[knot_num].1) as f32
                        / (positions[knot_num - 1].0 - positions[knot_num].0) as f32;
                    println!(
                        "slope: {slope}, rise: {}, run{}",
                        (positions[knot_num - 1].1 - positions[knot_num].1),
                        (positions[knot_num - 1].0 - positions[knot_num].0)
                    );

                    //positive slope right
                    if slope.is_sign_positive()
                        && (positions[knot_num - 1].0 > positions[knot_num].0)
                    {
                        println!("pos right");
                        positions[knot_num].0 += 1;
                        positions[knot_num].1 += 1;
                    }
                    //positive slope left
                    if slope.is_sign_positive()
                        && (positions[knot_num - 1].0 < positions[knot_num].0)
                    {
                        println!("pos left");
                        positions[knot_num].0 -= 1;
                        positions[knot_num].1 -= 1;
                    }
                    //negative slope right
                    if slope.is_sign_negative()
                        && (positions[knot_num - 1].0 > positions[knot_num].0)
                    {
                        println!("neg right");
                        positions[knot_num].0 += 1;
                        positions[knot_num].1 -= 1;
                    }
                    //negative slope left
                    if slope.is_sign_negative()
                        && (positions[knot_num - 1].0 < positions[knot_num].0)
                    {
                        println!("neg left");
                        positions[knot_num].0 -= 1;
                        positions[knot_num].1 += 1;
                    }
                }
            }
        }
        //we updated our tail positions, so lets check if we already visited this position, and update accordingly.
        visited.insert(positions[9]);
        //marked[tail_pos.1 as usize][tail_pos.0 as usize] = '#';
        //print_board(head_pos, tail_pos);
    }

    println!("bounds: x:{:?}\ny:{:?}", x_bound, y_bound);
    println!("visited: {}", visited.len());
    /*for line in marked.iter().rev() {
        for char in line {
            print!("{char}");
        }
        println!();
    }*/
}

fn print_board(head_pos: (i32, i32), tail_pos: (i32, i32)) {
    let mut board: Vec<Vec<char>> = vec![vec!['X'; 6]; 5];

    board[head_pos.1 as usize][head_pos.0 as usize] = 'H';
    board[tail_pos.1 as usize][tail_pos.0 as usize] = 'T';
    for line in board.iter().rev() {
        for char in line.iter() {
            if *char == 'H' {
                print!("{}", format!("{char}").on_green());
            } else if *char == 'T' {
                print!("{}", format!("{char}").on_red());
            } else {
                print!("{char}");
            }
        }
        println!();
    }
    println!();
}
