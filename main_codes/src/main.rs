#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::HashMap;
use std::io;
extern crate colored;
// extern crate sha1;
// extern crate lazy_static;
use colored::Colorize;
use sha1::{Digest, Sha1};
use lazy_static::lazy_static;
use std::sync::Mutex;


struct Account{
    username: String,
    password: String,
}

lazy_static! {
    static ref MAP: std::sync::Mutex<HashMap<String, String>> = {
        let mut map = HashMap::new();
        std::sync::Mutex::new(map)
    };
}

fn main(){

    println!("{}","\n+++ Authentication Server +++".bold().yellow().italic());

    println!("Do you want to Create Account or Login?\n>> C for Create Account\n>> L for Login");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Wrong input");

    match choice.trim().as_ref(){
        "C" => CreateAccount(),
        "c" => CreateAccount(),
        "L" => Login(),
        "l" => Login(),
        _ => panic(),
    }

}

fn CreateAccount(){
    println!("{}","++ Create Account ++".bold().green().italic());

    println!("Enter Username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Wrong input");
    let mut input = healthy_input(username.clone());

    if input == false{
        return CreateAccount()
    }

    println!("Enter Password: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Wrong input");
    input = healthy_input(password.clone());

    if input == false{
        return CreateAccount()
    }

    println!("Enter Password Again: ");
    let mut confirm_password = String::new();
    io::stdin().read_line(&mut confirm_password).expect("Wrong input");

    if password.trim() == confirm_password.trim(){
        println!("{}","\nCreating Account...\n".bold().italic());
        convert_to_sha(username.trim().to_string(), password.trim().to_string());
    }else{
        println!("{}","Password doesn't match, Tryagain".bold().red().italic());
        return CreateAccount()
    }
}

fn convert_to_sha(username: String, password: String){

    println!("{},{}", username, password);

    let mut hasher = Sha1::new();
    let mut hasher_password = Sha1::new();

    hasher.update(&username);
    let result = hasher.finalize();
    let result_str = format!("{:x}", result);

    hasher_password.update(&password);
    let result_password = hasher_password.finalize();
    let result_password_str = format!("{:x}", result_password);
    println!("{}\n{}",result_str, result_password_str);

    println!("{}","\n++ Account Created ++".bold().green().italic());

    append_to_map(result_str, result_password_str);
}

fn append_to_map(username: String, password: String){
    println!("{}","\n++ Append to Map ++".bold().italic());
    let mut map = MAP.lock().unwrap();
    map.insert(username, password);
    
    for (key, value) in &*map {
        println!("{}: {}", key, value);
    }
}

fn panic(){
    println!("{}", "Wrong input...".red().bold());
    return main()
}

fn healthy_input(input: String) -> bool{
    if input.trim().is_empty(){
        println!("{}", "Wrong input... TryAgain".red().bold());
        false
    }
    else{
        true
    }
}




fn Login(){
    println!("Login");
}

// fn read_map() {
//     println!("{}","++ Append to Map ++".bold().green().italic());
//     let map = MAP.lock().unwrap();
//     for (key, value) in map.iter() {
//         println!("{}: {}", key, value);
//     }
// }
