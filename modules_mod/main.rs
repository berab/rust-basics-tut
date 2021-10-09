mod trial {
    fn chicken(){
        println!("Chicken!"); // this doesnt have to be pub since chicken wont be used outside of mod
    }

    pub fn print_this(){
        chicken(); 
        println!("sa beyler turk var mi?");
    }

    pub mod water {
    
        pub fn print_this() {
            println!("I am water.");
        }
    }
}

fn main() {
    trial::water::print_this();
}