use crate::Challange;

struct MovementCommand {
    location: Location,
    amount: u32,
}

#[derive(Debug)]
enum Location {
    Up,
    Down,
    Forward,
}



pub fn run_challange(input_data: &str, challange: Challange) {
    let movement_commands: Vec<MovementCommand> = create_commands(input_data);

    println!(
        "movement: {:?}, amount: {}",
        movement_commands[0].location, movement_commands[1].amount
    );

    let (horizontal_pos, depth) = move_submarine_2(&movement_commands, match challange{
        Challange::One => false,
        Challange::Two => true
    });

    println!("horizontal_position: {}, depth: {}", horizontal_pos, depth);
    let multiplied_position = horizontal_pos * depth;

    println!(
        "multiplied depth and horizontal_pos: {}",
        multiplied_position
    );
}

fn create_commands(commands_string: &str) -> Vec<MovementCommand> {
    let split = commands_string.lines();
    let mut commands: Vec<MovementCommand> = vec![];

    for s in split {
        let command_split: Vec<&str> = s.split_whitespace().collect();
        commands.push(MovementCommand {
            location: match command_split[0] {
                "up" => Location::Up,
                "down" => Location::Down,
                "forward" => Location::Forward,
                _ => panic!("command was not correct"),
            },
            amount: command_split[1].parse().unwrap(),
        });
    }

    commands
}

fn move_submarine_2(commands: &[MovementCommand], use_aim: bool) -> (u32, u32) {
    let mut horizontal_pos: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    for command in commands {
        match command.location {
            Location::Up => aim -= command.amount,
            Location::Down => aim += command.amount,
            Location::Forward => {
                horizontal_pos += command.amount;
                depth += if use_aim { aim * command.amount } else { 0 }
            }
        }
    }
    if !use_aim {
        depth = aim;
    }

    (horizontal_pos.to_owned(), depth.to_owned())
}
