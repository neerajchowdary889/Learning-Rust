struct Job{
    company: String,
    employeer: String,
    salary: i32
}

fn main(){
    let Job1 = Job{
        company: String::from("Microsoft"),
        employeer: String::from("Satyam"),
        salary: 120000
    };
    println!("Company is {}\nRecruter is {}\nSalary is {}\n",Job1.company, Job1.employeer, Job1.salary);

    let Job2 = Job{
        company : String::from("Google"),
        employeer : String::from("Sidhappa"),
        salary : 150000
    }; 
    println!("Company is {}\nRecruter is {}\nSalary is {}",Job2.company, Job2.employeer, Job2.salary);
}