use crate::Challange;

pub fn run_challange(input_data: &str, challange: Challange) {
    let input_lines: Vec<Vec<char>> = input_data
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.chars().collect())
        .collect();

    let mut illegal_items: Vec<char> = vec![];

    'outer_loop: for line in input_lines{
        let mut stack: Vec<char> = vec![];

        for char in line{
            let possibly_opposite_char = is_closing_char(&char);

            if possibly_opposite_char != char {
                if *stack.last().unwrap() == possibly_opposite_char {
                    stack.pop();
                }
                else{
                    illegal_items.push(char);
                    continue 'outer_loop;
                }
            }
            else{
                stack.push(char);
            }
        }

    }

    println!("score: {}", count_points(&illegal_items));

    
}

fn is_closing_char(char: &char) -> char{
    if *char == ')'{
        return '(';
    }
    else if *char == ']'{
        return '[';
    }
    else if *char == '}'{
        return '{';
    }
    else if *char == '>'{
        return '<'
    }
    else {
        return *char;
    } 
}

fn count_points(illegal_items: &[char]) -> u32 {
    let mut score : u32 = 0;
    for illegal_item in illegal_items{
        if *illegal_item == ')'{
            score += 3;
        }
        else if *illegal_item == ']'{
            score += 57;
        }
        else if *illegal_item == '}'{
            score += 1197;
        }
        else if *illegal_item == '>'{
            score += 25137;
        }
    }

    score
}
