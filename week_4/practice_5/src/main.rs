fn main() {
    let fullname = " Pan-Atlantic University ";
    println!();
    println!("Name:{}", fullname);
    println!();
    println!("Before trim");
    println!("length is {}",fullname.len());
    println!();
    println!("after trim");
    println!("length is {}",fullname.trim().len());
}
