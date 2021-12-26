use crate::Challange;
// use factorial;
pub fn run_challange(input_data: &str, challange: Challange) {
    let mut crabs: Vec<u32> = input_data
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    crabs.sort();

    let value_to_use = match challange{
        Challange::One => crabs[crabs.len() / 2],
        Challange::Two => (crabs.iter().map(|x| *x as f32).sum::<f32>() / crabs.len() as f32).floor() as u32
    };

    let mut fuel_needed: u32 = 0;
    for crab in crabs{
        let distance =  (value_to_use as i32 - crab as i32).abs() as u32;
        fuel_needed += match challange{
            Challange::One => distance,
            Challange::Two => distance * (distance + 1) / 2, 
        };
    }

    println!("average: {}, feul_needed: {}",value_to_use, fuel_needed);
}

