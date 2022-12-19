use std::fs;
pub fn main() {
    let file_contents = fs::read_to_string("./inputs/input7.txt").unwrap();
    let file_contents = file_contents.lines().collect::<Vec<&str>>();

    let mut file_system: Vec<Node> = Vec::new();
    let mut cur: usize = 0;
    file_system.push(Node::new());

    for line in file_contents {
        println!("\n{line}");
        let parts: Vec<&str> = line.split(" ").collect();
        println!("{:?}", parts);

        if line.chars().collect::<Vec<char>>()[0] == '$' {
            print!("command found - ");
            match parts[1] {
                "cd" => {
                    if file_system[cur].begun_counting {
                        file_system[cur].done_counting = true;
                    }
                    match parts[2] {
                        ".." => {
                            print!(" cd up - ");
                            //set our pointer up a level so long as we are not at root.
                            if cur != 0 {
                                cur = file_system[cur].parent.unwrap();
                            }
                        }
                        "/" => {
                            print!(" cd root - ");
                            cur = 0;
                        }
                        _ => {
                            print!(" cd in - ");
                            let child_pos = file_system[cur]
                                .children
                                .iter()
                                .position(|e| e.0 == parts[2]);
                            if child_pos.is_some() {
                                cur = file_system[cur].children[child_pos.unwrap()].1;
                            } else {
                                print!("switching to a dir that doesnt exist yet! - ");

                                //create the new node
                                let mut new_node = Node::new();
                                //link it to its parent
                                new_node.parent = Some(cur);
                                //put it in the memory arena
                                file_system.push(new_node);
                                let new_index = file_system.len() - 1;
                                //put the new node in the current's children vec
                                file_system[cur]
                                    .children
                                    .push((parts[2].to_string(), new_index));
                                //update the cur pointer to our new node
                                cur = new_index;
                            }
                        }
                    }
                }
                "ls" => {
                    print!(" list - ");
                    file_system[cur].begun_counting = true;
                }
                _ => panic!(" bad command - "),
            }
        } else {
            print!("data found - ");
            match parts[0] {
                "dir" => {
                    print!("this is a subdir");
                }
                _ => {
                    print!("this is a file");

                    if !file_system[cur].done_counting {
                        file_system[cur].size += parts[0].parse::<usize>().unwrap();
                    }
                }
            }
        }
    }

    for i in 0..file_system.len() {
        //leaf node. traverse upwards to root
        if file_system[i].children.is_empty() {
            let mut cur = i;
            while file_system[cur].parent.is_some() {
                let parent = file_system[cur].parent.unwrap();
                file_system[parent].size += file_system[cur].size;

                cur = parent;
            }
        }
    }

    for node in file_system.iter() {
        println!("{:?}", node);
    }

    file_system.sort_by(|a, b| a.size.cmp(&b.size));

    println!("sorted\n\n\n");
    for node in file_system.iter() {
        println!("{:?}", node);
    }

    let day_2 = file_system.clone();

    let i = file_system.partition_point(|x| x.size < 100000);
    let sum: usize = file_system[0..i].iter().map(|e| e.size).sum();
    println!("day 1 sum {sum}");

    let root = file_system.len() - 1;
    println!("root is size: {}", file_system[root].size);
    let free_space = 70000000 - file_system[root].size;
    println!("there are {free_space} units of free space");
    let del_min = 30000000 - free_space;
    let min_del = day_2.partition_point(|e| e.size < del_min);
    println!(
        "we need to del at least: {del_min}, {} is not big enough, {} is too big, we choose: {}",
        day_2[min_del - 1].size,
        day_2[min_del + 1].size,
        day_2[min_del].size
    );
}

#[derive(Debug, Clone)]
struct Node {
    parent: Option<usize>,
    children: Vec<(String, usize)>,
    size: usize,
    begun_counting: bool,
    done_counting: bool,
}

impl Node {
    pub fn new() -> Self {
        Node {
            parent: None,
            children: Vec::new(),
            size: 0,
            begun_counting: false,
            done_counting: false,
        }
    }
}
