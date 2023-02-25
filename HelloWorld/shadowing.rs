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
    }

    let var = var - 5;
    println!("Variable is {}", var);

    let var = "String";
    println!("Variable is {}", var);
}
