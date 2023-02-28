fn main(){
    println!("Hello world");

    // Expression: anything that can return.Example
    let number ={
        let x = 3; //This is statement.
        x + 1 //Expression, we need to return 4 so no semicolon.
    };
    // return expressions should be without semicolons.
    println!("{}",number);
    let addnumbers = add_numbers(10,555);
    println!("From the function add_numbrs is {}", addnumbers);
    let check = totalcheck(10,555);
    println!("From the function totalcheck is {}", check);
}

fn add_numbers(a: i32, b: i32) -> i32{
    a + b
    //return expression without semicolon.
    //we can also use return keyword also.
}

fn totalcheck(a: i32, b: i32) -> i32 {
    let result = a + b;
    let mut integer;
    if result >= 600{
        integer = result;
        return result;
    }
    else{
        return 0;
    }
}