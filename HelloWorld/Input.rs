use std::io;
pub fn main(){
    let maxtime = expected_minutes_in_oven();
    println!("Cooked time: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Wrong");
    let remaining: i32 = input.trim().parse().expect("Wrong");
    let timeleft = remaining_minutes_in_oven(remaining, maxtime);

    println!("Layers: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Wrong");
    let layers: i32 = input.trim().parse().expect("Wrong");
    let layersXtime = preparation_time_in_minutes(layers);
    elapsed_time_in_minutes(layersXtime, remaining);
}
pub fn expected_minutes_in_oven() -> i32 {
    println!("Input the total time to cook in minutes: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Wrong");
    let totaltime: i32 = input.trim().parse().expect("Wrong");
    return totaltime;
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32, maxtime: i32) -> i32 {
    let timeleft = maxtime - actual_minutes_in_oven;
    println!("Timeleft is {}",timeleft);
    return timeleft;
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let layersXtime: i32 = number_of_layers * 2;
    return layersXtime;
}

pub fn elapsed_time_in_minutes(layersXtime: i32, timebeen: i32) -> i32 {
    let all = layersXtime + timebeen;
    println!("All {}",all);
    return all;
}
