use colored::*;
use std::cmp::Ord;

pub fn main() {
    let input = include_str!("../../inputs/input12.txt");

    //turn the input string into a matrix of usizes representing the heighmap.
    //note down the source and dest indexes for future use
    let mut source = 0;
    let mut dest = 0;
    let map = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            let row_width = line.len();

            line.chars()
                .enumerate()
                .map(|(col_index, c)| {
                    if c == 'S' {
                        source = row_index * row_width + col_index;
                        1
                    } else if c == 'E' {
                        dest = row_index * row_width + col_index;

                        26
                    } else {
                        ((c as u32) - 96) as usize
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    //need this for index calculations since we will be dealing with a flattened map
    let width: i32 = map[0].len() as i32;
    //flatten the map to make it a bit easier to deal with
    let map: Vec<usize> = map.iter().flatten().map(|e| *e).collect();
    //lets keep track of the upper bound of indexes for this flattend map for checking neighbors
    let size: i32 = map.len() as i32;

    //this is going to be out actual graph structure for operating djikstras on
    let mut adj_matrix: Vec<Vec<bool>> = vec![vec![false; size as usize]; size as usize];

    for (index, value) in map.iter().enumerate() {
        //NORTH
        let north_index: i32 = (index as i32) - width;

        if north_index >= 0 && north_index < size {
            let north_value = map[north_index as usize];
            if north_value < *value || (north_value - (*value)) <= 1 {
                adj_matrix[index][north_index as usize] = true;
            }
        }

        //EAST
        let east_index: i32 = (index as i32) + 1;

        if east_index < size && ((east_index / width) == (index as i32 / width)) {
            let east_value = map[east_index as usize];
            if east_value < *value || (east_value - (*value)) <= 1 {
                adj_matrix[index][east_index as usize] = true;
            }
        }
        //SOUTH
        let south_index: i32 = (index as i32) + width;

        if south_index >= 0 && south_index < size {
            let south_value = map[south_index as usize];

            if south_value < *value || (south_value - (*value)) <= 1 {
                adj_matrix[index][south_index as usize] = true;
            }
        }
        //WEST
        let west_index: i32 = (index as i32) - 1;

        if west_index >= 0 && ((west_index / width) == (index as i32 / width)) {
            let west_value = map[west_index as usize];

            if west_value < *value || (west_value - (*value)) <= 1 {
                adj_matrix[index][west_index as usize] = true;
            }
        }
    }

    ///////////////DJIKSTRAS ALG

    //backing data structures for djikstras
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

    'search: while !q.is_empty() {
        //fake a min heap lmfao
        //yes this has horrible runtime. i know. leave me alone.
        q.sort_by(|a, b| {
            let a_dist = dist[*a];
            let b_dist = dist[*b];
            b_dist.cmp(&a_dist)
        });

        //get our min dist node to look at
        let u = q.pop().unwrap();
        /*println!(
            "popping: {},{}: {}: {} -- {} nodes remain",
            (u as i32) / width,
            (u as i32) % width,
            char::from_u32((map[u] + 96) as u32).unwrap(),
            dist[u],
            q.len()
        );*/

        //for each neighbor v of u still in Q:
        for (v, exists) in adj_matrix[u].iter().enumerate() {
            if *exists && q.contains(&v) {
                /*println!(
                    "looking at neighbor: {},{}",
                    v / width as usize,
                    v % width as usize
                );*/

                //break early once we find dest, but make sure we update our backing structs
                if v == dest {
                    prev[v] = u;
                    break 'search;
                }
                let alt = dist[u] + 1;
                if alt < dist[v] {
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

    //turn the 1d map back into a 2d map for pretty printing
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

    ///////////////////////////////////PART 2//////////////////////////////////////////////////////
    //we can cheat a little bit so we are going to. notice that in our input there is a full column of bs and those are the only bs.
    //since we must have a b to elevate up from an a, we can restrict our search space to start at these bs and search from there using an implied extra step to get to an adjacent A

    //keep track of our best found path as a (start index,cost) 2-tuple
    let mut min_path: (usize, usize) = (0, usize::MAX);

    //start at each b in the first column
    for start in (1..size as usize).step_by(width as usize) {
        println!(
            "searching from: {},{}",
            start / width as usize,
            start % width as usize
        );
        //backing data structures for djikstras
        let mut dist: Vec<i32> = vec![i32::MAX; size as usize];
        let mut prev: Vec<usize> = vec![usize::MIN; size as usize];
        dist[start] = 0;

        let mut q = (0..(size as usize)).collect::<Vec<usize>>();

        'search: while !q.is_empty() {
            //fake a min heap lmfao
            //yes this has horrible runtime. i know. leave me alone.
            q.sort_by(|a, b| {
                let a_dist = dist[*a];
                let b_dist = dist[*b];
                b_dist.cmp(&a_dist)
            });

            //get our min dist node to look at
            let u = q.pop().unwrap();
            /*println!(
                "popping: {},{}: {}: {} -- {} nodes remain",
                (u as i32) / width,
                (u as i32) % width,
                char::from_u32((map[u] + 96) as u32).unwrap(),
                dist[u],
                q.len()
            );*/

            //for each neighbor v of u still in Q:
            for (v, exists) in adj_matrix[u].iter().enumerate() {
                if *exists && q.contains(&v) {
                    /*println!(
                        "looking at neighbor: {},{}",
                        v / width as usize,
                        v % width as usize
                    );*/

                    //break early once we find dest, but make sure we update our backing structs
                    if v == dest {
                        prev[v] = u;
                        break 'search;
                    }
                    let alt = dist[u] + 1;
                    if alt < dist[v] {
                        dist[v] = alt;
                        prev[v] = u
                    }
                }
            }
        }

        //ok now we want to analyze our shortest path
        let mut path: Vec<usize> = Vec::new();
        let mut cur = dest;
        while cur != start {
            path.push(cur);
            cur = prev[cur];
        }
        if path.len() < min_path.1 {
            min_path.0 = start;
            min_path.1 = path.len();
        }
    }
    println!("best path is : {:?}", min_path);
    println!("dont forget the actual answer is one higher than this!!");
}
