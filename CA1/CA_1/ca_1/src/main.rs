// Rust program that writes the names for patients information

use std::io;

fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();  
   let mut input4 = String::new();
   let mut input5 = String::new();
   let mut input6 = String::new();
   let mut input7 = String::new();
   let mut input8 = String::new();

   println!("Enter your name:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your DOB");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let DOB:i32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your email address:");
    let special_characrer = "@"; //default
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let email_address:i32 = input3.trim().parse().expect("Not a valid number");

    println!("Enter your phone number");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let phone_number:i32 = input4.trim().parse().expect("Not a valid number");

    println!("Enter your number of siblings");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let number_of_siblings:i32 = input5.trim().parse().expect("Not a valid number");

    println!("Enter number of children");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let number_of_children:i32 = input6.trim().parse().expect("Not a valid number");
   
   println!("Enter your Medical diagnosis");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let medical_diagnosis:i32 = input7.trim().parse().expect("Not a valid number");


    println!("Enter vilage of residence");
     io::stdin().read_line(&mut input8).expect("Not a valid string");
    let village_of_residence:i32 = input8.trim().parse().expect("Not a valid number");

   if medical_diagnosis = "Alzhemier" && DOB >= 50 && number_of_children >= 4 &&  village_of_residence = "Akpabom" 
   {
   println!("The user gets a discount of 20% discount");
   }

}

     


    
