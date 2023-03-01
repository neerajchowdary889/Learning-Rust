static mut key: i8 = 56;
fn main(){
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