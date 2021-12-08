use crate::Challange;

#[derive(Debug)]
struct BingoBoard {
    values: [[u32; 5]; 5],
    checked_of_values: [[bool; 5]; 5],
    bingo_time: u32,
    last_checked_value: u32,
}

pub fn run_challange(input_data: &str, challange: Challange) {
    let (bingo_input, mut bingo_boards) = create_input(input_data);

    let best_bingo_board = find_best_or_worst_board(&mut bingo_boards, &bingo_input, match challange{
        Challange::One => true,
        Challange::Two => false
    });

    let score = calculate_bingo_score(&best_bingo_board);

    println!("score is: {:?}", score);
}

fn create_input(input_data: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let lines: Vec<&str> = input_data.lines().collect();

    let bingo_input: Vec<u32> = lines[0]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let bingo_boards = create_bingo_boards(&lines[2..]);

    (bingo_input, bingo_boards)
}

fn create_bingo_boards(input: &[&str]) -> Vec<BingoBoard> {
    let mut bingo_boards: Vec<BingoBoard> = vec![];
    let mut bingo_board = BingoBoard {
        values: [[0; 5]; 5],
        checked_of_values: [[false; 5]; 5],
        bingo_time: 1000,
        last_checked_value: 0
    };

    for i in 0..5 {
        let mut j: usize = 0;
        for item in input[i].split_whitespace() {
            bingo_board.values[i][j] = item.parse::<u32>().unwrap();
            j += 1;
        }
    }

    bingo_boards.push(bingo_board);

    if input.len() >= 6 {
        bingo_boards.extend(create_bingo_boards(&input[6..]));
    }

    bingo_boards
}

fn find_best_or_worst_board(boards: &mut [BingoBoard], input: &[u32], find_best_board: bool) -> BingoBoard {
    let mut best_bingo_board: BingoBoard = BingoBoard {
        values: [[0; 5]; 5],
        checked_of_values: [[false; 5]; 5],
        bingo_time: if find_best_board{1000} else{0},
        last_checked_value: 0

    };

    let mut last_checked_value: u32 = 0;

    // for k in input {
    'outer_loop: for board in boards {
        for k in 0..input.len() {
            for i in 0..board.values.len() {
                for j in 0..board.values[i].len() {
                    if input[k] == board.values[i][j] {
                        board.checked_of_values[i][j] = true;
                    }
                }
            }
            if has_bingo(&board.checked_of_values) {
                board.bingo_time = k as u32;
                if best_bingo_board.bingo_time > board.bingo_time && find_best_board {
                    best_bingo_board.bingo_time = board.bingo_time;
                    best_bingo_board.values = board.values;
                    best_bingo_board.checked_of_values = board.checked_of_values;
                    best_bingo_board.last_checked_value = input[k] as u32;
                } 
                if best_bingo_board.bingo_time < board.bingo_time && !find_best_board{
                    best_bingo_board.bingo_time = board.bingo_time;
                    best_bingo_board.values = board.values;
                    best_bingo_board.checked_of_values = board.checked_of_values;
                    best_bingo_board.last_checked_value = input[k] as u32;
                }
                continue 'outer_loop;
            }
        }
    }

    best_bingo_board
}


fn has_bingo(checked_values: &[[bool; 5]; 5]) -> bool {
    'check_row: for i in 0..5 {
        for j in 0..5 {
            if !checked_values[i][j] {
                continue 'check_row;
            }
        }

        return true;
    }
    'check_column: for i in 0..5 {
        for j in 0..5 {
            if !checked_values[j][i] {
                continue 'check_column;
            }
        }

        return true;
    }

    false
}

fn calculate_bingo_score(bingo_board: &BingoBoard) -> u32 {
    let mut sum_unmarked_values = 0;

    for i in 0..5 {
        for j in 0..5 {
            if !bingo_board.checked_of_values[i][j] {
                sum_unmarked_values += bingo_board.values[i][j];
            }
        }
    }

    let score = sum_unmarked_values * bingo_board.last_checked_value;
    score
}
