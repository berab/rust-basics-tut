mod random_info;
use random_info::{RandomInfo, SomeTrait};

#[allow(dead_code)]
#[derive(Debug)]
struct MyData {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
    random: RandomInfo
}

impl RandomInfo {
    pub fn is_larger (&self, compare_to: i64) -> bool {
        self.some_int > compare_to
    }
}

impl SomeTrait for MyData {
    fn is_valid(&self) -> bool {
        true
    }
}
#[allow(dead_code)]
fn print_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Yay!");
    }
} // we combine diffeernt structs (my data and random info) and impl under a same subject (trait) here. using traits

impl Default for MyData {
    fn default() -> Self {
        Self {
            some_bool: true,
            some_float: 10.3,
            some_int: 80,
            random: RandomInfo::new(true)
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let mut random_info_var = RandomInfo {
        call_count: 0,
        some_bool: true,
        some_int: 10
    };
    let mut first_var = MyData::default();

    let is_this_smaller = random_info_var.is_smaller(2); // !! lower case self (&self) makes it a dot(.) operator. Capital case self (Self) makes it two colons(::) operator. !!
    let is_this_larger = random_info_var.is_larger(20);
    let is_valid = random_info_var.is_valid();

    first_var.some_int = 120;

    let second_var = MyData{
        some_int: 12,
        ..first_var
    }; 
    // Gives a partial move error :/ but after Copy macro, it works.

    println!("{:?}", first_var);
    print_if_is_valid(&random_info_var);
    print_if_is_valid(&first_var)
}