trait SomeCustomTrait {
    fn blah_blah(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct MyStruct {
    something: i32,
}

impl SomeCustomTrait for MyStruct {
    fn blah_blah(&self, a: &str, b:&str) -> String{
        self.something.to_string() + " - " + a + " - " + b
    }
}

impl SomeCustomTrait for i32 {
    fn blah_blah(&self, a: &str, b:&str) -> String{
        "i32".to_string() + " - " + a + " - " + b
    }
}

// fn my_func<T>(input_a:T, input_b: T) -> T 
// where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug {
//     println!("input_a has {:?}", input_a);
//     input_a - input_b
// }

fn main() {
    let test = MyStruct { something: 1000};
    let result = do_this(&test);

    let testi32 = 18;
    let result2 = do_this(&testi32);
}

fn do_this<T>(some_var: &T) -> String
where T: SomeCustomTrait + std::fmt::Debug {
    println!("{:?}", some_var);
    some_var.blah_blah("first", "second")
}