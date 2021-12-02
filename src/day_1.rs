use crate::Challange;

pub fn run_challange(input_data: &str, challange: Challange ){
    let depth_readings: Vec<u32> = generate_int_array(input_data);

    let larger_depth_mesurements: u32 = calculate_depth_increased(
        &match challange{
            Challange::One => depth_readings,
            Challange::Two => calculate_measurement_window(&depth_readings)
        }
    );

    println!("amount of increasing depths: {}", larger_depth_mesurements);
}

fn generate_int_array(data: &str) -> Vec<u32> {
    let split = data.split('\n');
    let mut values: Vec<u32> = vec![];

    for s in split {
        let value: u32 = s.parse().unwrap();
        values.push(value);
    }

    values
}

fn calculate_depth_increased(depth_readings: &[u32]) -> u32 {
    let mut latest_depth: u32 = 0;

    let mut larger_depth_mesurements: u32 = 0;
    for &depth in depth_readings {
        if latest_depth != 0 && depth > latest_depth {
            larger_depth_mesurements += 1;
        }
        latest_depth = depth;
    }

    larger_depth_mesurements
}

fn calculate_measurement_window(depth_readings: &[u32]) -> Vec<u32> {
    let mut measurement_windows: Vec<u32> = vec![];
    let mut n: u32 = 0;
    while n < depth_readings.len() as u32 - 2 {
        let mut three_measurements = 0;

        for i in n..n + 3 {
            three_measurements += depth_readings[i as usize];
        }
        measurement_windows.push(three_measurements);

        n += 1;
    }

    measurement_windows
}