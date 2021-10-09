struct Person {
    name: String,
    age: u8
}

trait HasVoiceBox {
    // speak
    fn speak(&self);
    //check if can speak
    fn can_speak(&self) --> bool;
}

impl HasVoiceBox for Person{
    fn speak(&self){
        println!("My name is {}", self.name);
    }

    fn can_speak(&self){
        if self.age > 0 {
            return true;
        } return false;
    }
}

fn main(){
    let p1 = Person {name: String::from("Beran"), age: 25};

    println!("Can {} speak?: {}", p1.name, p1.can_speak());
    // p1.speak();
}