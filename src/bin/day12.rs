use colored::*;
use std::cmp::Ord;
use std::cmp::Ordering;

pub fn main() {
    let input = include_str!("../../inputs/input12.txt");

    let map = input
        .lines()
        .map(|l| {
            //let new = l.strip_suffix("\n").unwrap();

            l.chars()
                .map(|c| {
                    if c == 'S' {
                        0
                    } else if c == 'E' {
                        27
                    } else {
                        ((c as u32) - 96) as usize
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    //need this for index calculations
    let width: i32 = map[0].len() as i32;
    //flatten the map to make it a bit easier to deal with
    let map: Vec<usize> = map.iter().flatten().map(|e| *e).collect();

    //lets keep track of the upper bound of indexes for this flattend map
    let size: i32 = map.len() as i32;

    //this is going to be out actual graph structure for operating djikstras on
    let mut adj_matrix: Vec<Vec<bool>> = vec![vec![false; size as usize]; size as usize];

    let mut source = 0;
    let mut dest = 0;

    for (index, value) in map.iter().enumerate() {
        if *value == 0 {
            println!(
                "source is at {},{}",
                (index as i32) / width,
                (index as i32) % width
            );

            source = index
        }
        if *value == 27 {
            println!(
                "dest is at {},{}",
                (index as i32) / width,
                (index as i32) % width
            );

            dest = index
        }
        /////////////this is just for our silly pretty little printing/////////////////////////
        /*if index % (width as usize) == 0 {
            println!();
        }
        print!(
            "{}",
            format!("{}", char::from_u32((value + 96) as u32).unwrap()).on_truecolor(
                (*value as u8) * 9,
                (255 - (value * 9)) as u8,
                0
            )
        );*/
        ///////////////////////////////////////////////////////////////////////////////////////

        /*println!(
            "node: {},{}, has neighbors to the:",
            (index as i32) / width,
            (index as i32) % width
        );*/

        //NORTH
        let north_index: i32 = (index as i32) - width;
        /*println!(
            "north is at {},{}",
            (north_index as i32) / width,
            (north_index as i32) % width
        );*/
        //only deal with the north node if it exists
        if north_index >= 0 && north_index < size {
            let north_value = map[north_index as usize];
            //println!("north value is: {north_value}");
            if north_value < *value || (north_value - (*value)) <= 1 {
                //println!("{}", format!("north").green());
                adj_matrix[index][north_index as usize] = true;
            }
        }

        //EAST
        let east_index: i32 = (index as i32) + 1;
        /*println!(
            "east is at {},{}",
            (east_index as i32) / width,
            (east_index as i32) % width
        );*/
        if east_index < size && ((east_index / width) == (index as i32 / width)) {
            let east_value = map[east_index as usize];
            //println!("east value is: {east_value}");
            if east_value < *value || (east_value - (*value)) <= 1 {
                //println!("{}", format!("east").green());
                adj_matrix[index][east_index as usize] = true;
            }
        }
        //SOUTH
        let south_index: i32 = (index as i32) + width;
        /*println!(
            "south is at {},{}",
            (south_index as i32) / width,
            (south_index as i32) % width
        );*/
        if south_index >= 0 && south_index < size {
            let south_value = map[south_index as usize];
            //println!("south value is: {south_value}");

            if south_value < *value || (south_value - (*value)) <= 1 {
                //println!("{}", format!("south").green());
                adj_matrix[index][south_index as usize] = true;
            }
        }
        //WEST
        let west_index: i32 = (index as i32) - 1;
        /*println!(
            "west is at {},{}",
            (west_index as i32) / width,
            (west_index as i32) % width
        );*/
        if west_index >= 0 && ((west_index / width) == (index as i32 / width)) {
            let west_value = map[west_index as usize];
            //println!("west value is: {west_value}");

            if west_value < *value || (west_value - (*value)) <= 1 {
                //println!("{}", format!("west").green());
                adj_matrix[index][west_index as usize] = true;
            }
        }
        //println!();
    }

    /*for line in adj_matrix.iter() {
        for c in line {
            if *c {
                print!("T")
            } else {
                print!("F")
            }
        }
        println!();
    }*/

    //ok at this point we're going to pretend we designed a perfect adj matrix and just start doing the alg

    //backing data structures
    let mut dist: Vec<i32> = vec![i32::MAX; size as usize];
    let mut prev: Vec<usize> = vec![usize::MIN; size as usize];
    dist[source] = 0;

    //WHY THE FUCK IS THIS A MAX HEAP KYS
    //ok a pqueue WOULD be a more effective method but because binary heap is a max heap by default
    //, i would need to create a nother sturct and implment ord on it just to get a fucking min heap. so fuck that
    /*let q = (0..(size as usize))
    .map(|e| Node { id: e })
    .collect::<std::collections::BinaryHeap<Node>>();*/

    let mut q = (0..(size as usize)).collect::<Vec<usize>>();

    'life: while !q.is_empty() {
        //fake a min heap lmfao
        q.sort_by(|a, b| {
            let a_dist = dist[*a];
            let b_dist = dist[*b];
            b_dist.cmp(&a_dist)
        });

        /*for node in q.iter() {
            print!("{},", dist[*node]);
        }
        println!();*/
        //get our min dist node to look at
        let u = q.pop().unwrap();
        println!(
            "popping: {},{}: {}: {} -- {} nodes remain",
            (u as i32) / width,
            (u as i32) % width,
            char::from_u32((map[u] + 96) as u32).unwrap(),
            dist[u],
            q.len()
        );

        //for each neighbor v of u still in Q:
        for (v, exists) in adj_matrix[u].iter().enumerate() {
            if *exists && q.contains(&v) {
                println!(
                    "looking at neighbor: {},{}",
                    v / width as usize,
                    v % width as usize
                );

                if v == dest {
                    println!("OOGABOOGA");
                    prev[v] = u;
                    break 'life;
                }
                let alt = dist[u] + 1;
                if alt < dist[v] {
                    //println!("beepboop");
                    dist[v] = alt;
                    prev[v] = u
                }
            }
        }
    }

    //ok now we want to analyze our shortest path
    let mut path: Vec<usize> = Vec::new();
    let mut cur = dest;
    while cur != source {
        path.push(cur);
        cur = prev[cur];
    }

    let path: Vec<(usize, usize)> = path
        .iter()
        .map(|e| ((*e as i32 / width) as usize, (*e as i32 % width) as usize))
        .collect();

    println!("path is {:?}\n{}", path, path.len());

    //let mut pretty: Vec<Vec<char>> = vec![vec!['.'; width as usize]; map.len() / width as usize];
    let map: Vec<Vec<usize>> = map[..].chunks(width as usize).map(|e| e.to_vec()).collect();

    for (row, line) in map.iter().enumerate() {
        for (col, value) in line.iter().enumerate() {
            if path.contains(&(row, col)) {
                print!(
                    "{}",
                    format!("{}", char::from_u32((value + 96) as u32).unwrap()).on_bright_magenta()
                )
            } else {
                print!(
                    "{}",
                    format!("{}", char::from_u32((value + 96) as u32).unwrap()).on_truecolor(
                        (*value as u8) * 9,
                        (255 - (value * 9)) as u8,
                        0
                    )
                )
            }
        }
        println!();
    }
}
