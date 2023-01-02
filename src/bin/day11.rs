use apint::ApInt;
use std::collections::VecDeque;
pub fn main() {
    let contents = include_str!("../../inputs/input11.txt");
    let contents = contents.split("\n\n").collect::<Vec<&str>>();
    let mut monkeys: Vec<Monkey> = contents
        .iter()
        .map(|m| {
            println!("parsing: \n{m}");

            let lines = m.lines().collect::<Vec<&str>>();
            let id = lines[0].chars().collect::<Vec<char>>()[7]
                .to_digit(10)
                .unwrap();
            let mut items = lines[1]
                .split(",")
                .map(|e| e.chars().filter(|c| c.is_digit(10)).collect::<String>())
                .map(|e| e.parse::<u32>().unwrap())
                .map(|e| {
                    let n = ApInt::from_u32(e);
                    n.into_zero_resize(128)
                })
                .collect::<VecDeque<ApInt>>();

            let op = lines[2].split(" ").collect::<Vec<&str>>();
            let hack = lines[2].split("*").collect::<Vec<&str>>();
            let l = op.len();

            let op: (char, ApInt) = (
                op[l - 2].chars().collect::<Vec<char>>()[0],
                if hack.len() > 1 && hack[1] == " old" {
                    //println!("ooga booga");
                    //(-1).try_into().unwrap()
                    ApInt::zero(apint::BitWidth::new(128).unwrap())
                } else {
                    op[l - 1].parse::<i32>().unwrap().try_into().unwrap()
                },
            );

            let new_op: (Op, ApInt) = (
                match op.0 {
                    '+' => Op::Addition,
                    '*' => Op::Multiplication,
                    '-' => Op::Division,
                    '/' => Op::Division,
                    _ => panic!("bad op"),
                },
                op.1.into_zero_resize(128),
            );
            /*let test = lines[3].chars().collect::<Vec<char>>()[21]
            .to_digit(10)
            .unwrap();*/
            let test = lines[3].split(" ").collect::<Vec<&str>>();
            let l = test.len();
            let test: ApInt =
                ApInt::from_u32(test[l - 1].parse::<u32>().unwrap()).into_zero_resize(128);
            let True = lines[4].chars().collect::<Vec<char>>()[29]
                .to_digit(10)
                .unwrap();

            let False = lines[5].chars().collect::<Vec<char>>()[30]
                .to_digit(10)
                .unwrap();

            Monkey {
                Id: id as usize,
                Items: items,
                Operation: new_op,
                Test: test,
                True: True as usize,
                False: False as usize,
                num: 0,
            }
        })
        .collect();

    println!("init monkeys: ");

    for m in monkeys.iter() {
        println!("{:?}", m)
    }
    println!();

    let mut lcm = ApInt::from_u128(1);
    for m in 0..monkeys.len() {
        lcm *= &monkeys[m].Test;
    }

    for i in 1..=10000 {
        //println!("round: {i}");

        for m in 0..monkeys.len() {
            //println!("round: {}, monkey: {}", i, m);
            //println!("{:?}", monkeys[m]);

            //println!("Monkey {m}");

            //run this monkey through
            while !monkeys[m].Items.is_empty() {
                //inspect
                let mut item = monkeys[m].Items.pop_front().unwrap();
                monkeys[m].num += 1;
                //println!("\tMonkey inspects an item with a worry level of {:X}", item);

                match monkeys[m].Operation.0 {
                    Op::Addition => {
                        if monkeys[m].Operation.1.is_zero() {
                            item += &item.clone()
                        } else {
                            item += &monkeys[m].Operation.1
                        };
                        //print!("\t\tWorry level increases by {:X}", rhs);
                        //item.checked_add_assign(&rhs.clone()).unwrap();
                        //println!("to {:X}", item);
                    }
                    Op::Multiplication => {
                        if monkeys[m].Operation.1.is_zero() {
                            item *= &item.clone();
                        } else {
                            item *= &monkeys[m].Operation.1
                        };
                        item = item.into_wrapping_urem(&lcm).unwrap();
                        //print!("\t\tWorry level is multiplied by {:X} ", rhs);
                        //item.checked_mul_assign(&rhs.clone()).unwrap();
                        //item += &rhs;
                        //println!("to {:X}", item);
                    }
                    _ => panic!("unimplemented op"),
                }

                //divide because bored
                //item = item.into_checked_udiv(&ApInt::from_u32(3)).unwrap();
                //test n pass

                //let indx = if item % monkeys[m].Test == 0 {
                let indx = if item
                    .clone()
                    .into_wrapping_urem(&(monkeys[m].Test))
                    .unwrap()
                    .is_zero()
                {
                    monkeys[m].True
                } else {
                    monkeys[m].False
                };

                /*println!(
                    "\t\tItem with worry level {:X} is thrown to monkey {}.",
                    item, m
                );*/
                monkeys[indx].Items.push_back(item)
            }
        }

        if i == 1
            || i == 20
            || i == 1000
            || i == 2000
            || i == 3000
            || i == 4000
            || i == 5000
            || i == 6000
            || i == 7000
            || i == 8000
            || i == 9000
            || i == 10000
        {
            println!("round: {i}");
            for m in 0..monkeys.len() {
                println!("monkey {}: {}\n", monkeys[m].Id, monkeys[m].num)
            }
        }
    }

    monkeys.sort_by(|a, b| a.num.cmp(&b.num));

    println!("\n\n\n");
    for m in monkeys {
        println!("monkey {}: {}\n", m.Id, m.num)
    }
}

#[derive(Debug)]
struct Monkey {
    Id: usize,
    Items: VecDeque<ApInt>,
    Operation: (Op, ApInt),
    Test: ApInt,
    True: usize,
    False: usize,
    num: usize,
}

#[derive(Debug)]
enum Op {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
