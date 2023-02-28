enum Direction{
    up,
    down,
    left,
    right
}

fn main(){
    let playerdirection:Direction = Direction::up;
    let playerdirection1:Direction = Direction::down;

    match playerdirection{
        Direction::up => println!("We are heading up"),
        Direction::down => println!("We are heading down"),
        Direction::left => println!("We are heading left"),
        Direction::right => println!("We are heading right")
    }
    match playerdirection1{
        Direction::up => println!("We are heading up"),
        Direction::down => println!("We are heading down"),
        Direction::left => println!("We are heading left"),
        Direction::right => println!("We are heading right")
    }
}