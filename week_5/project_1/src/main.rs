// Finding the roots of a quadratic equation 

use std::io;

fn main(){
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("Enter value for a:");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let a:i32 = input1.trim().parse().expect("Not a valid number");

   println!("Enter value for b:");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:i32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value for c");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:i32 = input3.trim().parse().expect("Not a valid number");

    let d:i32 = (b * b) - (4 * a * c);

    if d > 0 {
       println!("There are two distinct roots");
    }
    if d < 0 {
       println!("There are no real roots");
    }
     if d == 0 {
       println!("There is exactly one real root"); 

    }

}






