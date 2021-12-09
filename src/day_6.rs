use crate::Challange;

pub fn run_challange(input: &str, challange: Challange){
    let mut lanterfishes = create_lanterfish_timers(input);

    let_days_pass(&mut lanterfishes, 256);

    println!("fish after 80 days {}", count_total_fishes(&lanterfishes));
}

fn create_lanterfish_timers_inefficient(input: &str) -> Vec<u32>{
    let mut lanterfish_timers: Vec<u32> = vec![];

    for lanterfish_timer in input.split(','){
        lanterfish_timers.push(lanterfish_timer.parse::<u32>().unwrap());

    }

    lanterfish_timers
}

fn create_lanterfish_timers(input: &str) -> [u64; 9]{

    return input.split(',').map(|x| x.parse::<u64>().unwrap()).fold([0;9], |mut lanterfishes, timer| {
        lanterfishes[timer as usize] += 1;
        lanterfishes
    });

}

fn let_days_pass_unefficient(lanterfish_timers: &mut Vec<u32>, days_to_pass: u32) -> Vec<u32>{

    let mut new_lanterfish_timers: Vec<u32> = vec![];
    if days_to_pass == 0 {
        return lanterfish_timers.to_owned();
    }

    for lanterfish_timer in lanterfish_timers.iter_mut(){
        if *lanterfish_timer == 0 {
            *lanterfish_timer = 6;
            new_lanterfish_timers.push(8);
            continue;
        }
        
        *lanterfish_timer -= 1;
    }

    lanterfish_timers.extend(new_lanterfish_timers);

    let_days_pass_unefficient(lanterfish_timers, days_to_pass - 1)
}

fn let_days_pass(lanterfishes: &mut [u64; 9], days_to_pass: u32) {
    if days_to_pass == 0{
        return;
    }

    let fishes_to_give_birth = lanterfishes[0];

    for i in 1..lanterfishes.len(){
        lanterfishes[i - 1] = lanterfishes[i];
    }

    lanterfishes[6] += fishes_to_give_birth;
    lanterfishes[8] = fishes_to_give_birth;

    let_days_pass(lanterfishes, days_to_pass - 1)

}

fn count_total_fishes(lanterfishes: &[u64; 9]) -> u64{
    let mut total_fishes = 0;
    for fish in lanterfishes{
        total_fishes += fish;
    }

    total_fishes
}