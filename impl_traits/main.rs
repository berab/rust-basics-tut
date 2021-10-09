struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    fn to_string(&self) -> String{
        return format!("My name is {} and I am {}.", self.name, self.age);
    }
}

// fn asdsd(x: &Person) {
//     println!("My name is {} and I am {}.", x.name, x.age);
// }

fn main() {
    let beran = Person {name: String::from("Beran") , age: 25};
    // asdsd(&beran);
    println!("{}", beran.to_string());
}