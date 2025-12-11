const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: i32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let instructions: Vec<&str> = INPUT.split("\n").collect();

    let instructions: Vec<Instruction> = instructions
        .into_iter()
        .map(|instruction| {
            let (direction, count): (&str, &str) = instruction.split_at(1);

            let direction: Direction = match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction {direction}"),
            };

            let count: i32 = count.parse::<i32>().unwrap();

            Instruction { direction, count }
        })
        .collect();

    let mut number: i32 = 50;

    let mut password: u32 = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Left => {
                for _ in 0..instruction.count {
                    number -= 1;

                    match number {
                        0..=99 => {}
                        -1 => number = 99,
                        ..-1 | 100.. => panic!("Number out of bounds: {number}"),
                    }

                    if number == 0 {
                        password += 1;
                    }
                }
            }
            Direction::Right => {
                for _ in 0..instruction.count {
                    number += 1;

                    match number {
                        100 => number = 0,
                        0..=99 => {}
                        ..0 | 101.. => panic!("Number out of bounds: {number}"),
                    }

                    if number == 0 {
                        password += 1;
                    }
                }
            }
        }
    }

    println!("{password}");
}
