fn main(){
    //Typecasting i64 to i32 or i32 to i64.

    let x = 12700 as i64;
    let y = 10 as i32;
    //if we directly add x and y then i throws error before its different datatypes.
    let z = x + (y as i64);
    println!("{}",z);

    {
        let x = (i32::MAX as i64) +1;
        // i32::MAX return the max number of i32.
        let y = 78 as i32;

        let z = x + (y as i64);
        println!("{}",z);

    }
}