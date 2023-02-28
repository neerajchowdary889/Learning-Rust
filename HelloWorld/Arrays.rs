#![allow(dead_code)]
use std::io;
fn main(){
    let arr: [u8; 5] = [1,2,3,4,5];
    println!("{:?} ---> {}", arr, arr.len());

    // Unlike python we can't append elements to the list.
    // we need to create one duplicate list and add element to it.
    let mut array = ["AIE","CSE","ECE","EEE","ELC"];
    println!("{:?}",array);
    println!("Array index 4 --> {}",array[3]);
    array[3] = "BCE";
    println!("Array index 4 --> {}",array[3]);
    vectors();
}

fn vectors(){

    let mut vector: Vec<i64> = vec![1,2,3,4,5];
    println!("{:?}",vector);
    vector.push(6);
    println!("6 pushed to vector");

    println!("Type you input:");
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Wrong input");
    let vec: i64 = input.trim().parse().expect("Something Wrong");

    vector.push(vec);
    println!("{:?}",vector);
    
    println!("\n Removing 6 from list");
    vector.remove(5);
    println!("{:?}",vector);
}