use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_10;


pub enum Challange{
    One,
    Two
}

fn main() {

    let filepath = read_filepath();
    let input_data: String = load_file(&filepath);

    //Day 1
    // day_1::run_challange(&input_data, Challange::One);
    // day_1::run_challange(&input_data, Challange::Two);

    //Day 2
    // day_2::run_challange(&input_data, Challange::One);
    // day_2::run_challange(&input_data, Challange::Two);

    //Day 3
    // day_3::run_challange(&input_data, Challange::One);
    // day_3::run_challange(&input_data, Challange::Two);

    //Day 4
    // day_4::run_challange(&input_data, Challange::One);
    // day_4::run_challange(&input_data, Challange::Two);

    //Day 5
    // day_5::run_challange(&input_data, Challange::One);
    // day_5::run_challange(&input_data, Challange::Two);

    //Day 6
    // day_6::run_challange(&input_data, Challange::One);
    // day_6::run_challange(&input_data, Challange::Two);

    //Day 10
    day_10::run_challange(&input_data, Challange::One);
    day_10::run_challange(&input_data, Challange::Two);





}

fn read_filepath() -> String{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Enter only a filepath after the run command");
    }
    args[1].to_owned()
}

fn load_file(filepath: &str) -> String {
    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("could not read {}: {}", display, why),
        Ok(file) => file,
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content){
        Err(why) => panic!("could not read {}: {}", display, why),
        Ok(_) => println!("loaded {}", display),
    }

    file_content.trim().to_string()
}

