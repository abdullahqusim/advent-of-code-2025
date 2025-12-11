const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: u32,
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

            let count: u32 = count.parse::<u32>().unwrap();

            Instruction { direction, count }
        })
        .collect();

    let mut number: u32 = 50;

    let mut password: u32 = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Left => {
                while number < instruction.count {
                    number += 100
                }

                number -= instruction.count;
            }
            Direction::Right => {
                number += instruction.count;

                while number > 99 {
                    number -= 100
                }
            }
        }

        if number == 0 {
            password += 1;
        }
    }

    println!("{password}");
}
