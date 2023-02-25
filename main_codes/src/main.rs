fn main() {
    // This is Exterior scope.
    let var = 65;
    println!("Variable is {}", var);

    {
        //Shadowing
        // Information inside the brackets is called Interior scope.
        /* it depend on previous variables but change done in this brackets doesn't reflect
        in original */
        let var = var - 25;
        println!("Variable is {}", var);

        /* For easy readability of large numbers, we can use a visual 
        separator _ underscore to separate digits. That is 50,000 can be 
        written as 50_000 . */
        let secondvar = 11_000.555_001;
        println!("Variable is {}", secondvar);
        
        let secondvar = 55_001;
        println!("Variable is {}", secondvar);
    }

    let var = var - 5;
    println!("Variable is {}", var);

    let var = "String";
    println!("Variable is {}", var);

    let var = "🤷‍♀️";
    println!("EMoji is {}", var);
}