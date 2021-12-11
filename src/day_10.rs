use crate::Challange;

pub fn run_challange(input_data: &str, challange: Challange) {
 
    let mut input_lines: Vec<Vec<char>> = input_data
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.chars().collect())
        .collect();

    let error_score = remove_illegal_items(&mut input_lines);
    println!("error score: {}", error_score);

    if matches!(challange, Challange::Two){
        let completion_scores = fix_incomplete_lines(&mut input_lines);

        println!(
            "middle autocomplete score: {}",
            completion_scores[completion_scores.len() / 2]
        );
    }
    
}

fn remove_illegal_items(input_lines: &mut Vec<Vec<char>>) -> u32 {
    let mut illegal_items: Vec<char> = vec![];

    let mut index_of_illegal_items: Vec<usize> = vec![];

    let mut line_index: usize = 0;

    'outer_loop: for line in input_lines.iter() {
        let mut stack: Vec<char> = vec![];

        for char in line {
            let possibly_opposite_char = is_closing_char(&char);

            if possibly_opposite_char != *char {
                if *stack.last().unwrap() == possibly_opposite_char {
                    stack.pop();
                } else {
                    illegal_items.push(*char);
                    index_of_illegal_items.push(line_index);
                    continue 'outer_loop;
                }
            } else {
                stack.push(*char);
            }
        }

        line_index += 1;
    }

    for index in index_of_illegal_items {
        input_lines.remove(index);
    }

    count_points_error(&illegal_items)
}

fn fix_incomplete_lines(input_lines: &mut Vec<Vec<char>>) -> Vec<u64> {
    let mut scores: Vec<u64> = vec![];
    for line in input_lines.iter() {
        let mut line_score: u64 = 0;
        let mut stack: Vec<char> = vec![];

        for char in line {
            if is_opening_char(char) {
                stack.push(*char);
            } else {
                stack.pop();
            }
        }

        for char in stack.iter().rev() {
            count_points_auto_correct(&mut line_score, &char);
        }

        scores.push(line_score);
    }

    scores.sort();

    scores
}

fn is_closing_char(char: &char) -> char {
    if *char == ')' {
        return '(';
    } else if *char == ']' {
        return '[';
    } else if *char == '}' {
        return '{';
    } else if *char == '>' {
        return '<';
    } else {
        return *char;
    }
}

fn is_opening_char(char: &char) -> bool {
    if *char == '(' || *char == '[' || *char == '{' || *char == '<' {
        true
    } else {
        false
    }
}

fn count_points_error(illegal_items: &[char]) -> u32 {
    let mut score: u32 = 0;
    for illegal_item in illegal_items {
        if *illegal_item == ')' {
            score += 3;
        } else if *illegal_item == ']' {
            score += 57;
        } else if *illegal_item == '}' {
            score += 1197;
        } else if *illegal_item == '>' {
            score += 25137;
        }
    }

    score
}

fn count_points_auto_correct(score: &mut u64, char: &char) {
    *score *= 5;
    if *char == '(' {
        *score += 1;
    } else if *char == '[' {
        *score += 2;
    } else if *char == '{' {
        *score += 3;
    } else if *char == '<' {
        *score += 4;
    } else {
        panic!("wrong char given");
    }
}
