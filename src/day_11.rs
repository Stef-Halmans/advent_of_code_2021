use crate::Challange;
pub fn run_challange(input_data: &str, challange: Challange) {
    let mut octopuses: Vec<Vec<i32>> = input_data
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| {
            x.chars()
                .collect::<Vec<char>>()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let mut highlighted_score = 0;
    for i in 0..match challange {
        Challange::One => 100,
        Challange::Two => 1000,
    } {
        highlighted_score += perform_one_step(&mut octopuses);
        if matches!(challange, Challange::Two) {
            if all_flashing(&mut octopuses) {
                println!("all octuposes flash at: {} ", i + 1);
                print_octopuses(&octopuses);
                break;
            }
        }
    }
    if matches!(challange, Challange::One) {
        println!("score after 80 days: {}", highlighted_score);
    }
}

fn perform_one_step(octopuses: &mut Vec<Vec<i32>>) -> u32 {
    let mut flashes = 0;
    for i in 0..octopuses.len() {
        for j in 0..octopuses[i].len() {
            if flash(&mut octopuses[i as usize][j as usize]) {
                // *highlighted_score += 1;flashes +=
                flash_adjecent_octopuses(octopuses, i as i32, j as i32);
            }
        }
    }

    for i in 0..octopuses.len() {
        for j in 0..octopuses[i].len() {
            if octopuses[i][j] > 9 {
                flashes += 1;
                octopuses[i][j] = 0;
            }
        }
    }
    flashes
}

fn print_octopuses(octopuses: &Vec<Vec<i32>>){
    
    for i in 0..octopuses.len() {
        for j in 0..octopuses[i].len() {
            print!("{}", octopuses[i][j]);
        }
        println!();
    }
}

fn all_flashing(octopuses: &mut Vec<Vec<i32>>) -> bool {
    let mut all_are_flashing = true;
    for i in 0..octopuses.len() {
        for j in 0..octopuses[i].len() {
            if octopuses[i][j] != 0 {
                all_are_flashing = false;
            }
        }
    }
    all_are_flashing
}

fn flash(energie_level: &mut i32) -> bool {
    *energie_level += 1;
    if *energie_level == 10 {
        true
    } else {
        false
    }
}

fn flash_adjecent_octopuses(octopuses: &mut Vec<Vec<i32>>, i: i32, j: i32) {
    for i_new in i - 1..=i + 1 {
        for j_new in j - 1..=j + 1 {
            if (i_new == i && j_new == j)
                || i_new < 0
                || j_new < 0
                || i_new >= octopuses.len() as i32
                || j_new >= octopuses[i as usize].len() as i32
            {
                continue;
            }
            if flash(&mut octopuses[i_new as usize][j_new as usize]) {
                flash_adjecent_octopuses(octopuses, i_new, j_new);
            }
        }
    }
}
