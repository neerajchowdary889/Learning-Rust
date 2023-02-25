fn main(){
    let x = 89;
    // Here every variable by default is immuttable in rust
    // you need to assign mut to make variable muttable 
    println!("Data Variable is: {}",x);
    
    let mut y = "HelloBro";
    println!("{}",y);

    y = "OkBro";
    println!("{}",y);

    //By overwriting we can change x value, just creating other variable with same name as x ex:
    let x = 65;
    println!("Data Variable is: {}",x);

    let x = x+5;
    println!("Data Variable is: {}",x);
}