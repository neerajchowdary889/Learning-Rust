use std::io;
fn main(){
    let vec: Vec<String> = vec!["AIE".to_string(),"CSE".to_string(),"ECE".to_string(),"EEE".to_string(),"ELC".to_string()];
    println!("{:?}", vec);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let int = index(vec.clone(), input.trim().to_string());
    if int == -1{
        println!("{} is not in the list", input.trim());
    }else{
    println!("Main --> {} is at index {}, testing {}", input, int, vec[int as usize]);
    }
}

fn index(vec: Vec<String>, input: String) -> i32{
    for i in 0..vec.len(){
        if vec[i] == input{
            let j = i as i32;
            println!("{} is at index {}", input, i);
            return j;
        }
    }
    return -1 as i32;
}