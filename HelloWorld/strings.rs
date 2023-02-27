// static mut STRINGS: Option<String> = Some("Hello".to_string());

// fn main() {
//     // String literal
//     let company: &str = "Microsoft";
//     let place: &str = "Hyderabad";

//     println!("Company is {}\nPlace is {}", company, place);

//     unsafe {
//         // Mutable string
//         STRINGS = Some("Govindha".to_string());

//         let state: &'static str = "Telangana";
//         let cm: &'static str = "KCR";
//         println!("State is {}\nCM is {}", state, cm);

//         println!("Static string is {}", STRINGS.as_ref().unwrap());
//     }

//     not_main();
// }

// fn not_main() {
//     unsafe {
//         println!("Static string is {}", STRINGS.as_ref().unwrap());
//     }
// }
