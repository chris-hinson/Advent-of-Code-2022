pub fn main() {
    let input = include_str!("../../inputs/input10.txt");
    let program: Vec<Vec<&str>> = input
        .lines()
        .map(|e| e.split(" ").collect::<Vec<&str>>())
        .collect();

    let mut cpu = Cpu {
        cycle: 0,
        X: 1,
        framebuffer: vec![vec!['.'; 40]; 6],
    };
    let mut day1: i32 = 0;

    for command in program {
        //println!("{}", command[0]);
        match command[0] {
            "noop" => {
                if cpu.cycle % 40 == 20 {
                    day1 += cpu.cycle as i32 * cpu.X;
                }

                cpu.tick();
            }
            "addx" => {
                if cpu.cycle % 40 == 20 {
                    day1 += cpu.cycle as i32 * cpu.X;
                }
                //tick
                cpu.tick();
                //tick
                cpu.tick();

                //complete and set
                cpu.X += command[1].parse::<i32>().unwrap();
            }
            _ => panic!("bad opcode"),
        }
    }

    println!("day 1: {day1}");

    for line in cpu.framebuffer {
        for char in line {
            print!("{char}");
        }
        println!();
    }
}

struct Cpu {
    cycle: i32,
    X: i32,
    framebuffer: Vec<Vec<char>>,
}

impl Cpu {
    pub fn tick(&mut self) {
        let dot = self.cycle % 40;
        //draw for the current cycle
        if (dot - 1..=dot + 1).contains(&self.X) {
            self.framebuffer[(self.cycle / 40) as usize][(self.cycle % 40) as usize] = '#';
        }

        //tick our clock
        self.cycle += 1;
    }
}
