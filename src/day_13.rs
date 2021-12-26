use crate::Challange;
use std::collections::HashMap;

pub fn run_challange(input_data: &str, challange: Challange) {
    let input_lines = input_data.lines();
    let mut are_points = true;
    let mut points: HashMap<(u32, u32), u32> = HashMap::new();
    let mut fold_commands: Vec<(String, u32)> = vec![];

    for line in input_lines {
        if line.is_empty() {
            are_points = false;
        } else if are_points {
            let point: Vec<&str> = line.split(',').collect();
            let tuple = (point[0].parse::<u32>().unwrap(), point[1].parse().unwrap());
            points.entry(tuple).or_insert(1);
        } else if !are_points {
            let fold_command: Vec<&str> = line.split('=').collect();
            fold_commands.push((
                String::from(fold_command[0].replace("fold along", "").trim()),
                fold_command[1].parse().unwrap(),
            ));
        }
    }

    let mut max_values: (u32, u32) = (
        points.iter().map(|x| x.0 .0).max().unwrap(),
        points.iter().map(|x| x.0 .1).max().unwrap(),
    );
    println!("{:?}", max_values);

    let runs: usize = match challange {
        Challange::One => 1,
        Challange::Two => fold_commands.len(),
    };

    for i in 0..runs {
        execute_fold(&fold_commands[i], &mut points, &mut max_values);
    }
    println!("amount of points: {}", points.len());

    print(&points, &max_values);
}

fn execute_fold(
    fold_command: &(String, u32),
    points: &mut HashMap<(u32, u32), u32>,
    max_size: &mut (u32, u32),
) {
    println!("{:?}", fold_command);
    if fold_command.0.eq("x") {
        fold_around_x(fold_command.1, points, &mut max_size.0);
    } else if fold_command.0.eq("y") {
        fold_around_y(fold_command.1, points, &mut max_size.1);
    } else {
        println!("failure")
    }
}

fn fold_around_x(location: u32, points: &mut HashMap<(u32, u32), u32>, max_size: &mut u32) {
    let mut points_to_change: Vec<(u32, u32)> = vec![];

    for point in points.keys() {
        if point.0 > location {
            points_to_change.push((point.0, point.1));
        }
    }

    for point in points_to_change {
        points.entry((*max_size - point.0, point.1)).or_insert(1);
        points.remove(&point);
    }
    *max_size = location - 1;
}

fn fold_around_y(location: u32, points: &mut HashMap<(u32, u32), u32>, max_size: &mut u32) {
    let mut points_to_change: Vec<(u32, u32)> = vec![];
    for point in points.keys() {
        if point.1 > location {
            points_to_change.push((point.0, point.1));
        }
    }
    for point in points_to_change {
        points.entry((point.0, *max_size - point.1)).or_insert(1);
        points.remove(&point);
    }
    *max_size = location - 1;
}

fn print(points: &HashMap<(u32, u32), u32>, max_size: &(u32, u32)) {
    let mut field: Vec<Vec<bool>> =
        vec![vec![false; (max_size.0 + 1) as usize]; (max_size.1 + 1) as usize];
    for point in points {
        println!("{:?}", point);
        field[(point.0 .1) as usize][(point.0 .0) as usize] = true;
    }

    for row in field {
        for item in row {
            print!("{}", if item { "#" } else { "." });
        }
        println!();
    }
}
