use colored::*;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::cmp::max;

pub fn main() {
    let input = include_str!("../../inputs/input14_ex.txt");
    //println!("{input}");
    let mut contents: Vec<Vec<((usize, usize), (usize, usize))>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|e| {
                    let nums = e
                        .split(",")
                        .map(|e| e.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    (nums[0], nums[1])
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .map(|e| {
            let mut lines: Vec<((usize, usize), (usize, usize))> = Vec::new();
            for i in 0..(e.len() - 1) {
                lines.push((e[i], e[i + 1]));
            }
            lines
        })
        .collect();

    //run a quick preprocessing pass to find our grid bounds

    /*for group in contents.iter() {
        println!("{:?}", group);
    }*/
    for group in contents.iter_mut() {
        group.sort_unstable_by(|a, b| max(b.0 .0, b.1 .0).cmp(&max(a.0 .0, a.1 .0)));
    }
    contents.sort_unstable_by(|a, b| a[0].0.cmp(&b[0].0));

    /*println!();
    for group in contents.iter() {
        println!("{:?}", group);
    }*/
    let num_groups = contents.len();
    let min_x = contents[0][0].0 .0;
    let fin_len = contents[num_groups - 1].len();
    let max_x = max(
        contents[num_groups - 1][fin_len - 1].0 .0,
        contents[num_groups - 1][fin_len - 1].1 .0,
    );
    println!("minx: {min_x}, maxx: {max_x}");

    //
    /*for group in contents.iter() {
        println!("{:?}", group);
    }*/
    for group in contents.iter_mut() {
        group.sort_unstable_by(|a, b| max(b.1 .1, b.0 .1).cmp(&max(a.0 .1, a.1 .1)));
    }
    contents.sort_unstable_by(|a, b| a[0].1.cmp(&b[0].1));

    //println!();
    for group in contents.iter() {
        println!("{:?}", group);
    }
    let max_y = contents[0][0].0 .1;
    let fin_len = contents[num_groups - 1].len();
    let min_y = contents[num_groups - 1][fin_len - 1].0 .1;
    println!("miny: {min_y}, maxy: {max_y}");

    let width = max_x - min_x;
    let height = max_y - min_y;
    println!("width: {width}, height: {height}");
    let mut board = vec![vec![false; width]; height];

    let mut buffer: Vec<u32> = vec![0x00FF0000; (width * 5) * (height * 5)];

    let mut window = Window::new(
        "Test - ESC to exit",
        width * 5,
        height * 5,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }*/

        let mut rand = rand::thread_rng();

        /*let row = rand.gen_range(0..board.len());
        let col = rand.gen_range(0..board[0].len());
        board[row][col] = true;*/

        board[1][0] = true;
        board[1][1] = true;

        map_to_buffer(&board, &mut buffer, 0x0000FF00);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width * 5, height * 5)
            .unwrap();
    }
}

pub fn map_to_buffer(board: &Vec<Vec<bool>>, buffer: &mut Vec<u32>, color: u32) {
    for (r, row) in board.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col {
                let start_index = (r * 25 * board[0].len()) + (c * 5);
                for i in 0..5 {
                    for j in 0..5 {
                        buffer[start_index + (i * (board[0].len()) * 5) + j] = color;
                    }
                }
            }
        }
    }
}
