fn main(){
    // Tuples are immutable by default.
    let tuple: (u8, bool, f32, &str) = (5,true,2.1,"str");
    println!("1st -- {}\n2nd -- {}\n3rd -- {}\n4th -- {}",tuple.0,tuple.1,tuple.2,tuple.3);
    println!("{:?}",tuple);
    {
        let mut tuples = ("Neeraj", 18, 'M');
        println!("Name {}\nAge {}\nGender {}",tuples.0,tuples.1,tuples.2);
        tuples.1 = 19 ;
        println!("\nName {}\nAge {}\nGender {}",tuples.0,tuples.1,tuples.2);
    }
}