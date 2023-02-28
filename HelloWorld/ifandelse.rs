// same datatypes should be compared.
// we can't compare float with integer.
use std::io;
fn main() {
    /* let cond = 2 <= 2.2; This will throw error because can't
    compare integer and float.*/
    let cond = 2_f32 <= 2.2;
    println!("{}", cond);
    operators(cond);
    if_else();
}

fn operators(cond: bool) {
    /*
    && = and
    || = or
    ! = not
    */
    let cond2 = cond && true;
    println!("Condition 2 {}", cond2);
}

fn if_else(){

    println!("_________________________\nInput Message: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Something went wrong");
    let integer: i32 = input.trim().parse().expect("Fishy");

    if {integer >= 0} && {integer <= 10}{
        println!("If condition {}",integer);
    }
    else if {integer > 10} && {integer <= 30} {

        println!("Else If condition {}",integer);
    }
    else{

        println!("Else condition");
    }
}