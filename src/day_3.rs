use crate::Challange;

pub fn run_challange(input_data: &str, challange: Challange) {
    let columns_of_rows: Vec<Vec<u8>> = create_rows(input_data);

    match challange {
        Challange::One => {
            let (gamma, epsilon_rate) = calculate_gamma_and_epsiton(&columns_of_rows);
            let power = gamma * epsilon_rate;
            println!("gamme: {}, epsilon_rate: {}", gamma, epsilon_rate);
            println!("power: {}", power);
        }
        Challange::Two => {
            let oxygen_bytes: Vec<Vec<u8>> = calculate_bits(&columns_of_rows, 0, true);

            let scruber_bytes: Vec<Vec<u8>> = calculate_bits(&columns_of_rows, 0, false);

            println!("oxygen in bits: {:?}", &oxygen_bytes[0]);
            println!("scrubber in bits: {:?}", &scruber_bytes[0]);
            let mut oxygen_bits = String::default();
            let mut scrubber_bits = String::default();

            for bit in &oxygen_bytes[0]{
                oxygen_bits.push(char::from_digit(*bit as u32, 10).unwrap());
            }

            for bit in &scruber_bytes[0]{
                scrubber_bits.push(char::from_digit(*bit as u32, 10).unwrap());
            }

            let oxygen = u32::from_str_radix(&oxygen_bits, 2).unwrap();
            let scrubber = u32::from_str_radix(&scrubber_bits, 2).unwrap();

            println!("oxygen: {}", &oxygen);
            println!("scrubber: {}", &scrubber);

            let maintenance = oxygen * scrubber;

            println!("maintenance: {}", maintenance);

        }
    }
}

fn create_rows(input_data: &str) -> Vec<Vec<u8>> {
    let mut columns_of_rows: Vec<Vec<u8>> = vec![];

    let row_strings = input_data.lines();
    for row_string in row_strings {
        let chars = row_string.chars();

        let mut column: Vec<u8> = vec![];
        for char in chars {
            column.push(char.to_digit(10).unwrap().try_into().unwrap());
        }

        columns_of_rows.push(column);
    }

    columns_of_rows
}

fn calculate_gamma_and_epsiton(columns_of_rows: &[Vec<u8>]) -> (u8, u8) {
    let mut gamma_bits: String = String::default();
    let mut epsilon_rate_bits = String::default();

    for i in 0..columns_of_rows[0].len() {
        let mut amount_1 = 0;
        let mut amount_0 = 0;

        for row in columns_of_rows {
            if row[i] == 1 {
                amount_1 += 1;
            } else if row[i] == 0 {
                amount_0 += 1;
            } else {
                panic!("row[i] is not 1 or 0");
            }
        }

        if amount_1 > amount_0 {
            gamma_bits.push('1');
            epsilon_rate_bits.push('0');
        } else {
            gamma_bits.push('0');
            epsilon_rate_bits.push('1');
        }
    }

    (
        u8::from_str_radix(&gamma_bits, 2).unwrap(),
        u8::from_str_radix(&epsilon_rate_bits, 2).unwrap(),
    )
}

fn calculate_bits(columns_of_rows: &[Vec<u8>], column_number: u32, most_common_ones: bool) -> Vec<Vec<u8>> {
    if columns_of_rows.len() == 1 {
        return columns_of_rows.to_owned();
    }

    let mut rows_with_1: Vec<Vec<u8>> = vec![];
    let mut rows_with_0: Vec<Vec<u8>> = vec![];

    for row in columns_of_rows {
        if row[column_number as usize] == 1 {
            rows_with_1.push(row.clone());
        } else {
            rows_with_0.push(row.clone());
        }
    }
    
    calculate_bits(
        if rows_with_0.len() > rows_with_1.len() && most_common_ones {
            &rows_with_0
        } else if rows_with_0.len() <= rows_with_1.len() && most_common_ones {
            &rows_with_1
        } else if rows_with_0.len() <=rows_with_1.len() && !most_common_ones{
            &rows_with_0
        } else{
            &rows_with_1
        },
        column_number + 1, 
        most_common_ones
    )
}
