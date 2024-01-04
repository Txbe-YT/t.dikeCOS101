struct computers {
    HP:f32,
    IBM:f32,
    Toshiba:f32,
    dell:f32,

}

impl Computers {
  fn sum(&self) -> f32 {
    self.hp * 3.0 + self.ibm * 3.0 + self.Toshiba * 3.0
}

}

fn main() {
    let cost = computers {
        HP: 650000.0,
        IBM: 755000.0,
        toshiba: 550000.0,
        dell: 850000.0,

    };

    println!("The total cost of all the computers is {}",cost.sum());

}
