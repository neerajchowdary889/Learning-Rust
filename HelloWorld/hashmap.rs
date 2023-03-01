use std::collections::HashMap;
#[derive(Debug)]
#[allow(dead_code)]
struct Student{
    name: String,
    age: i32,
    class : String,
}

fn main(){
    let mut names = HashMap::new();
    //Order is not guaranteed.
    names.insert("Neeraj", 18);
    names.insert("Neeraj1", 19);
    names.insert("Neeraj2", 20);
    names.insert("Neeraj3", 21);
    names.insert("Neeraj4", 22);

    for (key,value) in &names{
        println!("{}: {}", key, value);
    }
    student();
}

fn student(){
    let mut map = HashMap::new();
    map.insert(1, Student{name: "Neeraj".to_string(), age: 18, class: "AIE".to_string()});
    map.insert(2, Student{name: "Neeraj1".to_string(), age: 19, class: "AIE".to_string()});
    map.insert(3, Student{name: "Neeraj2".to_string(), age: 17, class: "AIE".to_string()});

    for (key,value) in &map{
        println!("{:?}: {:?}", key, value);
    }
}