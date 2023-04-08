pub struct PersonalInfo{
    pub Name: String,
    pub Age: i32,
    pub Relationstatus: String,
}

impl details for PersonalInfo{
    fn Information(&self) -> String{
        format!("Name: {}\nAge: {}\nRelation Status: {}",self.Name,self.Age,self.Relationstatus)
    }
    fn OnlyName(&self) -> String{
        format!("Name: {}\nFrom Trait",self.Name)
    }
}

pub struct ProfessionalInfo{
    pub Name: String,
    pub occupation: String,
    pub salary: i32,
}

impl details for ProfessionalInfo{
    fn Information(&self) -> String{ 
        format!("Name: {}\nOccupation: {}\nSalary: {}",self.Name,self.occupation,self.salary)
    }
    fn OnlyName(&self) -> String{
        format!("Name: {}\nFrom Trait",self.Name)
    }
}

pub trait details{
    fn Information(&self) -> String{
        String::from("If there is no impl for that struct, then this is default message")
    }
    fn OnlyName(&self) -> String{
        String::from("From trait")
    }
}

fn main(){
    let Personal = PersonalInfo{
        Name: String::from("Rustman"),
        Age: 18,
        Relationstatus: String::from("Single"),
    };

    let Professional = ProfessionalInfo{
        Name: String::from("Rustman"),
        occupation: String::from("Software Engineer"),
        salary: 120000,
    };
    
    println!("Personal Details: {}\n+++++++++++++++++++++++", Personal.Information());
    println!("Personal Details: {}\n+++++++++++++++++++++++", Personal.OnlyName());
     println!("Professional Details: {}", Professional.Information());
}
