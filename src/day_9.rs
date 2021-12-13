use crate::Challange;

pub fn run_challange(input: &str, challange: Challange) {
    let data: Vec<Vec<u32>> = input
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| {
            x.chars()
                .collect::<Vec<char>>()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();

    let mut lowest_points: Vec<u32> = vec![];
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let item = data[i][j];
            // print!("{}", item);
            let mut is_lowest = true;
            if i > 0 {
                if data[i][j] >= data[i - 1][j] {
                    is_lowest = false;
                }
            }
            if j > 0 {
                if data[i][j] >= data[i][j - 1] {
                    is_lowest = false;
                }
            }
            if i + 1 < data.len() {
                if data[i][j] >= data[i + 1][j] {
                    is_lowest = false;
                }
            }
            if j + 1 < data[i].len() {
                if data[i][j] >= data[i][j + 1] {
                    is_lowest = false;
                }
            }
            if is_lowest {
                lowest_points.push(data[i][j] + 1);
            }
        }
        // println!();
    }
    // println!("{:?}",lowest_points);
    let mut score = 0;
    for lowest_point in lowest_points {
        score += lowest_point;
    }
    println!("score: {}", score);
}
