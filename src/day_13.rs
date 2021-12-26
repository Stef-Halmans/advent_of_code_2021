use crate::Challange;
use std::collections::HashSet;
use std::collections::HashMap;
pub fn run_challange(input_data: &str, challange: Challange) {
    let input_lines = input_data.lines();
    let mut are_points = true;
    let mut points: Vec<(u32, u32)> = vec![];
    let mut fold_commands: Vec<(String, u32)> = vec![];
    for line in input_lines {
        if line.is_empty() {
            are_points = false;
        } else if are_points {
            let point: Vec<&str> = line.split(',').collect();
            points.push((point[0].parse::<u32>().unwrap(), point[1].parse().unwrap()));
        } else if !are_points {
            let fold_command: Vec<&str> = line.split('=').collect();
            fold_commands.push((
                String::from(fold_command[0].replace("fold along", "").trim()),
                fold_command[1].parse().unwrap(),
            ));
        }
    }

    let mut max_values: (u32, u32) = (
        points.iter().map(|x| x.0).max().unwrap(),
        points.iter().map(|x| x.1).max().unwrap(),
    );

    let runs: usize = match challange {
        Challange::One => 1,
        Challange::Two => fold_commands.len(),
    };

    for i in 0..runs {
        let old_max_values = (max_values.0, max_values.1);
        execute_fold(&fold_commands[i], &mut points, &mut max_values);
    }
    print(&points, &max_values);

    println!(
        "{}",
        points
            .iter()
            .map(|x| String::from(x.0.to_string() + &x.1.to_string()))
            .collect::<HashSet<_>>()
            .len()
    );
}

fn execute_fold(
    fold_command: &(String, u32),
    points: &mut Vec<(u32, u32)>,
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
    // let mut points_to_remove = HashMap::new();
    // for i in 0..points.len(){
    //     points_to
    // }
}


fn fold_around_x(location: u32, points: &mut Vec<(u32, u32)>, max_size: &mut u32) {
    for point in points {
        if point.0 > location {
            point.0 = *max_size - point.0;
        }
    }
    *max_size = *max_size - location;
}

fn fold_around_y(location: u32, points: &mut Vec<(u32, u32)>, max_size: &mut u32) {
    for point in points {
        if point.1 > location {
            point.1 = *max_size - point.1;
        }
    }
    *max_size = *max_size - location;
}

fn print(points: &[(u32, u32)], max_size: &(u32, u32)) {
    let mut field: Vec<Vec<bool>> =
        vec![vec![false; (max_size.1 + 1) as usize]; (max_size.0 + 1) as usize];
    for point in points {
        println!("{:?}", point);
        field[(point.0) as usize][(point.1) as usize] = true;
    }

    for row in field {
        for item in row {
            print!("{}", if item { "#" } else { "." });
        }
        println!();
    }
}
