#![feature(iter_advance_by)]
use colored::*;
pub fn main() {
    let input = include_str!("../../inputs/input8.txt");
    //println!("{input}");

    let forest: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut counter = 0;
    let mut max_scenery: (usize, (usize, usize)) = (0, (0, 0));

    for row in 0..forest.len() {
        //note this only works for a square grid
        //probably a more sound method using proper iterators
        for col in 0..forest[0].len() {
            let our_height = forest[row][col];

            //slice the current column out of the 2d array
            let mut cur_col = forest.iter().flatten();
            cur_col.advance_by(col).unwrap();
            let cur_col = cur_col
                .step_by(forest[0].len())
                .map(|e| *e)
                .collect::<Vec<usize>>();

            //println!("\n\n{}", format!("{row},{col}: {}", forest[row][col]).red());

            //check north--------------------------------------------------------------------------
            let next_north = &cur_col[0..row].iter().rev().position(|e| *e >= our_height);
            let north_block = next_north.is_some();

            /*println!(
                "{}",
                format!(
                    "up is: {:?}",
                    &cur_col[0..row].iter().rev().collect::<Vec<&usize>>()
                )
                .green()
            );*/
            //println!("next up is: {:?}", next_north);
            let north_scenery = if next_north.is_some() {
                next_north.unwrap() + 1
            } else {
                row
            };
            //println!("up scenery is {north_scenery}\n");

            //check left---------------------------------------------------------------------------
            let next_east = &forest[row][0..col]
                .iter()
                .rev()
                .position(|e| *e >= our_height);
            let east_block = next_east.is_some();

            /*println!(
                "{}",
                format!(
                    "left is: {:?}",
                    &forest[row][0..col].iter().rev().collect::<Vec<&usize>>()
                )
                .green()
            );*/

            //println!("next left is {:?}", next_east);
            let east_scenery = if next_east.is_some() {
                next_east.unwrap() + 1
            } else {
                col
            };
            //println!("left scenery is {east_scenery}\n");

            //check down---------------------------------------------------------------------------
            let next_south = &cur_col[(row + 1)..].iter().position(|e| *e >= our_height);
            let south_block = next_south.is_some();

            /*println!(
                "{}",
                format!(
                    "down is: {:?}",
                    &cur_col[(row + 1)..].iter().collect::<Vec<&usize>>()
                )
                .green()
            );*/
            //println!("next down is {:?}", next_south);
            let south_scenery = if next_south.is_some() {
                next_south.unwrap() + 1
            } else {
                (forest.len() - row) - 1
            };
            //println!("down scenery is {south_scenery}\n");

            //check west---------------------------------------------------------------------------
            let next_west = &forest[row][(col + 1)..]
                .iter()
                .position(|e| *e >= our_height);
            let west_block = next_west.is_some();

            /*println!(
                "{}",
                format!(
                    "right is: {:?}",
                    &forest[row][(col + 1)..].iter().collect::<Vec<&usize>>()
                )
                .green()
            );*/

            //println!("next right is {:?}", next_west);
            let west_scenery = if next_west.is_some() {
                next_west.unwrap() + 1
            } else {
                (forest[0].len() - col) - 1
            };
            //println!("right scenery is {west_scenery}\n");

            //-------------------------------------------------------------------------------------
            if !north_block || !east_block || !south_block || !west_block {
                //println!("THIS TREE IS VISIBLE!!!!!!!");
                counter += 1;
            }
            let total_scenery = north_scenery * south_scenery * west_scenery * east_scenery;
            /*println!(
                "{}",
                format!("total scenery score is: {total_scenery}").yellow()
            );*/

            //update scenery to new total if possible
            if total_scenery > max_scenery.0 {
                max_scenery.0 = total_scenery;
                max_scenery.1 = (row, col);
            }
        }
    }

    //1713
    println!("there are: {} trees visible", counter);
    //(268464, (14, 51))
    println!("most scenic house is at: {:?}", max_scenery);
}
