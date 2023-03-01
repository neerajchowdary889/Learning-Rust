use std::fs::File;
use std::io;
use std::io::Read;
static mut key: i8 = 56;
fn main(){
    recoverable();
    assertt();
    panicmessages(); 
}

fn panicmessages(){
    unsafe{
        if key != 100{

            panic!("Not Hundred");
        }
    }
}

fn assertt(){
    let temp: i8 = 56;
    unsafe{
        assert!(temp == key, "Something went wrong");
        // if we have temp == key then it will continue. else it throws 
        // that something went wrong error we typed.
    }
}

fn recoverable() -> Result<(), io::Error>{
    let f = File::open("Rough.txt");
    
    let mut f = match f{
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}