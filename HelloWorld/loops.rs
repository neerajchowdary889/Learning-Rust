fn main(){
    Loop();
    whileloop();
    forloop();
    forloopvect();
}

fn Loop(){
    //This is infinite loop.
    let mut n: i32 = 1;

    loop{

        if n%2 == 0{
            n = n+1;
            continue;
        }
        
        if n >15{
            break;
        }
        println!("Loop function: {}",n);

        n = n+1;
    }
}

fn whileloop(){
    let mut n: i32 = 1;
    
    while n<=15{

        if n%2 == 0{
            println!("whileloop function: {}",n);
        }
        else{
            n = n+1;
            continue;
        }
        n = n+1;
    }
}

fn forloop(){

    for i in 1..15{

        if i%2 == 0{
            println!("Forloop function: {}",i);
        }
        else{
            continue;
        }

    }
}

fn forloopvect(){
    let branches = vec!["AIE","CSE","ECE","EEE","ELC"];

    for i in branches.iter(){
        println!("Branch is {}",i);
    }
    for (index, i) in branches.iter().enumerate(){
        println!("Index of Branch {} is {}",i,index);
    }
    println!("{:?}",branches);
}
